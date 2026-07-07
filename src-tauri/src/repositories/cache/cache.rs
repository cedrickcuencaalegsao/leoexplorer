use crate::core::constant::{CACHE_FILE_NAME, CACHE_TEMP_FILE_NAME};
use crate::core::models::{
    appcachestate::AppCacheState, cachebuildprogress::CacheBuildProgress, cachedata::CacheData,
    deepsearchsettings::SupportedFileExtension, file_entry::FileEntry, settings::Settings,
};
use crate::repositories::path::PathService;

use num_cpus;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Emitter, Manager};
use threadpool::ThreadPool;

#[derive(Serialize)]
pub struct CacheStatus {
    pub status: String,
    pub message: Option<String>,
}

/// Service object that owns all cache state (status flag, status message,
/// and the in-memory cache tree) and exposes every cache operation as a
/// method. This plays the role a Java "CacheService" singleton class would:
/// one object, one place holding state, methods instead of loose functions.
pub struct CacheService {
    status: AtomicU8,
    message: Mutex<Option<String>>,
    global_cache: Mutex<Option<CacheData>>,
}

impl Default for CacheService {
    fn default() -> Self {
        CacheService {
            status: AtomicU8::new(0),
            message: Mutex::new(None),
            global_cache: Mutex::new(None),
        }
    }
}

/// Process-wide singleton, equivalent to a Java `CacheService.getInstance()`.
static CACHE_SERVICE: Lazy<CacheService> = Lazy::new(CacheService::default);

impl CacheService {
    pub fn instance() -> &'static CacheService {
        &CACHE_SERVICE
    }

    pub fn initialize_cache_directory(&self, app_handle: AppHandle) {
        let app_handle_clone = app_handle.clone();

        thread::spawn(move || {
            let service = CacheService::instance();
            if PathService::new().find_cache() {
                println!("Cache already exists. Using cached data.");
                service.set_cache_status(0, Some("Cache Loaded".to_string()));
                let _ = app_handle_clone
                    .emit("cache_ready", "Cache already exists. Using cached data.");
            } else {
                println!("Cache not found. Creating cache file.");
                let _ = app_handle_clone
                    .emit("creating_cache", "Cache not found. Creating cache file.");
                service.set_cache_status(1, Some("Creating cache file.".to_string()));
                service.create_cache(app_handle_clone);
            }
        });
    }

    fn ensure_cache_or_rebuild(&self, app: &AppHandle) -> bool {
        if Self::cache_file_path().exists() {
            return false;
        }

        if self.status.load(Ordering::SeqCst) == 1 {
            println!(
                "Cache file missing but a scan is already in progress; skipping rebuild trigger."
            );
            let _ = app.emit("creating_cache", "Cache rebuild already in progress.");
            return true;
        }

        println!("Cache file not found. Triggering fresh cache build.");
        self.set_cache_status(
            1,
            Some("Cache missing — rebuilding from scratch.".to_string()),
        );
        let _ = app.emit("creating_cache", "Cache file not found. Rebuilding cache.");

        if let Ok(mut guard) = self.global_cache.lock() {
            *guard = None;
        }
        self.create_cache(app.clone());
        true
    }

    pub fn update_cache_on_create(&self, app: AppHandle, new_path: PathBuf) {
        println!("update_cache_on_create called for: {:?}", new_path);

        let update_message = format!("Updating cache for: {}", new_path.display());
        self.set_cache_status(1, Some(update_message.clone()));
        if let Err(e) = app.emit("updating_cache", update_message.clone()) {
            eprintln!("Failed to emit updating_cache event: {:?}", e);
        }

        if self.ensure_cache_or_rebuild(&app) {
            return;
        }

        let mut cache: CacheData = match Self::read_cache_from_disk() {
            Ok(c) => c,
            Err(e) => {
                println!("Failed to parse cache: {}", e);
                self.set_cache_status(0, Some(format!("Failed to parse cache: {}", e)));
                if let Err(emit_err) =
                    app.emit("cache_error", format!("Failed to parse cache: {}", e))
                {
                    eprintln!("Failed to emit cache_error event: {:?}", emit_err);
                }
                return;
            }
        };

        let is_dir = new_path.is_dir();

        let entry = FileEntry::FromPath(&new_path, is_dir);

        println!("Created entry: {:?} | flags: {:?}", entry.name, entry.flag);

        if let Err(e) = app.emit(
            "cache_progress",
            format!("Adding {} to cache...", entry.name),
        ) {
            eprintln!("Failed to emit cache_progress event: {:?}", e);
        }

        if !Self::insert_into_tree(&mut cache.roots, &new_path, entry.clone()) {
            println!("Failed to insert into tree, adding as root");
            cache.roots.push(entry.clone());
            cache
                .roots
                .sort_unstable_by(|a, b| match (a.is_dir, b.is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                });
        } else {
            println!("Successfully inserted into tree");
        }

        self.update_global_cache(&cache);
        println!("Global in-memory cache updated for new path");

        if let Err(e) = app.emit("cache_progress", "Updating RAM cache...") {
            eprintln!("Failed to emit cache_progress event: {:?}", e);
        }

        let state: tauri::State<'_, AppCacheState> = app.state::<AppCacheState>();
        if let Ok(mut mem_cache) = state.data.lock() {
            let key = PathBuf::from(&entry.path);
            mem_cache.roots.insert(key, entry.clone());
            if let Some(parent) = new_path.parent() {
                let parent_key = parent.to_path_buf();
                mem_cache.roots.remove(&parent_key);
                println!(
                    "AppCacheState: added {:?}, invalidated parent {:?}",
                    entry.path, parent_key
                );
            } else {
                println!("AppCacheState in-memory cache updated for {:?}", entry.path);
            }
        } else {
            println!("Failed to lock AppCacheState; skipping in-memory state update");
        }

        if let Err(e) = app.emit("cache_progress", "Writing cache to NVMe...") {
            eprintln!("Failed to emit cache_progress event: {:?}", e);
        }

        let mut disk_ok = true;
        if let Err(e) = Self::write_cache_to_disk(&cache) {
            println!(
                "Failed to write cache to disk after in-memory update: {}",
                e
            );
            disk_ok = false;
            self.set_cache_status(0, Some(format!("Failed to write cache: {}", e)));
            if let Err(emit_err) = app.emit("cache_error", format!("Failed to write cache: {}", e))
            {
                eprintln!("Failed to emit cache_error event: {:?}", emit_err);
            }
        } else {
            println!("Cache file updated successfully on disk");
        }

        if disk_ok {
            self.set_cache_status(0, Some(format!("Cache updated: {}", new_path.display())));

            match app.emit(
                "cache_completed",
                format!("Cache updated successfully: {}", new_path.display()),
            ) {
                Ok(_) => println!("cache_completed event emitted successfully"),
                Err(e) => println!("Failed to emit cache_completed event: {:?}", e),
            }

            match app.emit(
                "cache_updated",
                format!("Added to cache: {}", new_path.display()),
            ) {
                Ok(_) => println!("cache_updated event emitted successfully"),
                Err(e) => println!("Failed to emit cache_updated event: {:?}", e),
            }
        }
    }

    pub fn update_cache_on_remove(&self, app: AppHandle, removed_path: PathBuf) {
        println!("Update_cache_on_remove called for: {:?}", removed_path);

        let update_message = format!("Removing cache for: {}", removed_path.display());
        self.set_cache_status(1, Some(update_message.clone()));
        if let Err(e) = app.emit("updating_cache", update_message.clone()) {
            eprintln!("Failed to emit updating_cache event: {:?}", e);
        }

        if self.ensure_cache_or_rebuild(&app) {
            return;
        }

        let mut cache: CacheData = match Self::read_cache_from_disk() {
            Ok(c) => c,
            Err(e) => {
                println!("Failed to parse cache: {}", e);
                self.set_cache_status(0, Some(format!("Failed to parse cache: {}", e)));
                if let Err(emit_err) =
                    app.emit("cache_error", format!("Failed to parse cache: {}", e))
                {
                    eprintln!("Failed to emit cache_error event: {:?}", emit_err);
                }
                return;
            }
        };

        let normalized_removed_path = FileEntry::NormalizePath(&removed_path);
        println!("🗑️ Removing entries matching: {}", normalized_removed_path);

        if let Err(e) = app.emit(
            "cache_progress",
            format!("Removing {} from cache...", removed_path.display()),
        ) {
            eprintln!("Failed to emit cache_progress event: {:?}", e);
        }

        let removed_count = Self::remove_from_tree(&mut cache.roots, &normalized_removed_path);
        println!("Removed {} entries from cache", removed_count);

        if removed_count == 0 {
            println!("No entries found matching: {}", normalized_removed_path);
        }

        self.update_global_cache(&cache);
        println!("Global in-memory cache updated for removed path");

        if let Err(e) = app.emit("cache_progress", "Updating RAM cache...") {
            eprintln!("Failed to emit cache_progress event: {:?}", e);
        }

        let state: tauri::State<'_, AppCacheState> = app.state::<AppCacheState>();
        if let Ok(mut mem_cache) = state.data.lock() {
            let keys_to_remove: Vec<PathBuf> = mem_cache
                .roots
                .keys()
                .filter(|key| {
                    let normalized_key = FileEntry::NormalizePath(key);
                    Self::path_matches_or_under(&normalized_key, &normalized_removed_path)
                })
                .cloned()
                .collect();

            for key in &keys_to_remove {
                mem_cache.roots.remove(key);
            }

            if let Some(parent) = removed_path.parent() {
                let parent_key = parent.to_path_buf();
                mem_cache.roots.remove(&parent_key);
                println!(
                    "AppCacheState: removed {} entries, invalidated parent {:?}",
                    keys_to_remove.len(),
                    parent_key
                );
            } else {
                println!(
                    "AppCacheState in-memory cache updated, removed {} entries",
                    keys_to_remove.len()
                );
            }
        } else {
            println!("Failed to lock AppCacheState; skipping in-memory state update");
        }

        if let Err(e) = app.emit("cache_progress", "Writing cache to NVMe...") {
            eprintln!("Failed to emit cache_progress event: {:?}", e);
        }

        let mut disk_ok = true;
        if let Err(e) = Self::write_cache_to_disk(&cache) {
            println!(
                "Failed to write cache to disk after in-memory update: {}",
                e
            );
            disk_ok = false;
            self.set_cache_status(0, Some(format!("Failed to write cache: {}", e)));
            if let Err(emit_err) = app.emit("cache_error", format!("Failed to write cache: {}", e))
            {
                eprintln!("Failed to emit cache_error event: {:?}", emit_err);
            }
        } else {
            println!("Cache file updated successfully on disk");
        }

        if disk_ok {
            self.set_cache_status(
                0,
                Some(format!("Cache updated: removed {}", removed_path.display())),
            );

            match app.emit(
                "cache_completed",
                format!(
                    "Cache updated successfully: removed {}",
                    removed_path.display()
                ),
            ) {
                Ok(_) => println!("cache_completed event emitted successfully"),
                Err(e) => println!("Failed to emit cache_completed event: {:?}", e),
            }

            match app.emit(
                "cache_updated",
                format!("Removed from cache: {}", removed_path.display()),
            ) {
                Ok(_) => println!("cache_updated event emitted successfully"),
                Err(e) => println!("Failed to emit cache_updated event: {:?}", e),
            }
        }
    }

    fn update_global_cache(&self, data: &CacheData) {
        if let Ok(mut guard) = self.global_cache.lock() {
            *guard = Some(data.clone());
        }
    }

    #[allow(dead_code)]
    pub fn get_root_from_global_cache(&self, normalized_root_path: &str) -> Option<FileEntry> {
        if let Ok(guard) = self.global_cache.lock() {
            if let Some(data) = guard.as_ref() {
                return data
                    .roots
                    .iter()
                    .find(|root| root.path == normalized_root_path)
                    .cloned();
            }
        }
        None
    }

    pub fn get_cached_data(&self) -> Result<CacheData, String> {
        let mut guard = self
            .global_cache
            .lock()
            .map_err(|_| "Failed to lock global cache".to_string())?;

        if let Some(data) = guard.as_ref() {
            return Ok(data.clone());
        }

        let data = Self::read_cache_from_disk()?;
        *guard = Some(data.clone());
        Ok(data)
    }

    fn set_cache_status(&self, flag: u8, msg: Option<String>) {
        self.status.store(flag, Ordering::SeqCst);
        if let Ok(mut lock) = self.message.lock() {
            *lock = msg;
        }
    }

    pub fn get_cache_status(&self) -> Result<CacheStatus, String> {
        let s = self.status.load(Ordering::SeqCst);
        let message = self.message.lock().map(|m| m.clone()).unwrap_or(None);
        let status = match s {
            1 => "scanning",
            0 => "idle",
            _ => "unknown",
        };

        Ok(CacheStatus {
            status: status.to_string(),
            message,
        })
    }

    pub fn validate_cache(&self) -> Result<String, String> {
        let cache_path = Self::cache_file_path();

        if !cache_path.exists() {
            return Err("Cache file does not exist".to_string());
        }

        let metadata =
            fs::metadata(&cache_path).map_err(|e| format!("Failed to read metadata: {}", e))?;

        if metadata.len() == 0 {
            return Err("Cache file is empty".to_string());
        }

        let cache_data = Self::read_cache_from_disk()
            .map_err(|e| format!("Failed to load cache: {}. The cache may be corrupted.", e))?;

        Ok(format!(
            "Cache is valid! {} root entries, {} bytes",
            cache_data.roots.len(),
            metadata.len()
        ))
    }

    pub fn regenerate_cache(&self, app_handle: AppHandle) -> Result<String, String> {
        let cache_path = Self::cache_file_path();
        let temp_path = Self::cache_temp_file_path();

        let _ = fs::remove_file(&cache_path);
        let _ = fs::remove_file(&temp_path);

        println!("Starting cache regeneration...");

        if let Ok(mut guard) = self.global_cache.lock() {
            *guard = None;
        }
        self.create_cache(app_handle);

        Ok("Cache regeneration started".to_string())
    }

    fn create_cache(&self, app_handle: AppHandle) {
        let _ = app_handle.emit("cache_started", "Starting cache scan...");
        self.set_cache_status(1, Some("Starting cache scan...".to_string()));

        let base_dir = PathService::new().base_path();

        if let Err(e) = fs::create_dir_all(&base_dir) {
            eprintln!("Failed to create cache directory {:?}: {}", base_dir, e);
            let _ = app_handle.emit("cache_error", format!("Failed to create directory: {}", e));
            self.set_cache_status(0, Some(format!("Failed to create directory: {}", e)));
            return;
        }

        #[cfg(target_os = "windows")]
        let drive_roots = Self::get_windows_drives();

        #[cfg(target_os = "macos")]
        let drive_roots = vec![PathBuf::from("/Users"), PathBuf::from("/Applications")];

        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        let drive_roots = vec![PathBuf::from("/home")];

        let max_depth = 4;
        let drive_roots_bg = drive_roots.clone();
        let app_bg = app_handle.clone();
        let total_roots: u32 = drive_roots_bg.iter().filter(|r| r.exists()).count() as u32;

        std::thread::spawn(move || {
            let service = CacheService::instance();
            let (tx, rx) = mpsc::channel::<Vec<PathBuf>>();

            let _ = rayon::ThreadPoolBuilder::new()
                .num_threads(num_cpus::get().max(1))
                .build_global();

            let pool_size = Self::cache_parallelism().max(2);
            let (num_cores, threads_per_core) = Self::read_thread_settings();
            let pool = ThreadPool::new(pool_size);

            println!(
                "Starting parallel cache scan: pool={} threads (settings {}×{}), Rayon={} logical CPUs; zstd write uses multi-threaded compression",
                pool_size,
                num_cores,
                threads_per_core,
                num_cpus::get()
            );
            let parallel_msg = format!(
                "Parallel scan: {} workers, {} CPU cores (Rayon + zstd MT)",
                pool_size,
                num_cpus::get()
            );
            let _ = app_bg.emit("cache_progress", parallel_msg.clone());

            let start = std::time::Instant::now();
            let total_found = Arc::new(AtomicUsize::new(0));
            let roots_done = Arc::new(AtomicUsize::new(0));

            Self::emit_cache_build_progress(
                &app_bg,
                CacheBuildProgress {
                    phase: "scanning".to_string(),
                    percent: 0,
                    message: parallel_msg,
                    drives_completed: Some(0),
                    total_drives: Some(total_roots),
                    paths_found: Some(0),
                    duration_seconds: Some(0.0),
                },
            );

            for root in drive_roots_bg.into_iter() {
                if root.exists() {
                    let txc = tx.clone();
                    let r = root.clone();
                    let app_worker = app_bg.clone();
                    let counter = total_found.clone();
                    let roots_done_c = roots_done.clone();
                    let tr = total_roots;

                    pool.execute(move || {
                        let _ =
                            app_worker.emit("cache_progress", format!("Scanning: {}", r.display()));
                        println!("Scanning drive: {}", r.display());

                        let mut local_results = Vec::with_capacity(10000);
                        local_results.extend(Self::scan_directory_parallel(&r, max_depth, 0));

                        let count = local_results.len();
                        counter.fetch_add(count, Ordering::Relaxed);

                        println!("Completed: {} - {} items", r.display(), count);
                        let _ = app_worker.emit(
                            "cache_progress",
                            format!("Completed: {} ({} items)", r.display(), count),
                        );

                        let k = roots_done_c.fetch_add(1, Ordering::SeqCst) + 1;
                        let paths = counter.load(Ordering::Relaxed) as u64;
                        let pct = Self::cache_build_scan_percent(k, tr);
                        Self::emit_cache_build_progress(
                            &app_worker,
                            CacheBuildProgress {
                                phase: "scanning".to_string(),
                                percent: pct,
                                message: format!("Completed: {} ({} items)", r.display(), count),
                                drives_completed: Some(k as u32),
                                total_drives: Some(tr),
                                paths_found: Some(paths),
                                duration_seconds: Some(start.elapsed().as_secs_f64()),
                            },
                        );

                        let _ = txc.send(local_results);
                    });
                }
            }

            drop(tx);
            pool.join();

            let estimated_total = total_found.load(Ordering::Relaxed);
            let mut scanned = Vec::with_capacity(estimated_total);

            for part in rx.iter() {
                scanned.extend(part);
            }

            let scan_duration = start.elapsed();
            let total = scanned.len();

            println!(
                "📊 Scan completed in {:.2}s - {} paths found",
                scan_duration.as_secs_f64(),
                total
            );
            println!("Building tree structure...");

            let build_msg = format!("Building tree from {} items...", total);
            let _ = app_bg.emit("cache_progress", build_msg.clone());
            Self::emit_cache_build_progress(
                &app_bg,
                CacheBuildProgress {
                    phase: "building_tree".to_string(),
                    percent: 68,
                    message: build_msg,
                    drives_completed: Some(total_roots),
                    total_drives: Some(total_roots.max(1)),
                    paths_found: Some(total as u64),
                    duration_seconds: Some(start.elapsed().as_secs_f64()),
                },
            );

            let tree_start = std::time::Instant::now();
            // build_tree_blazing_fast calls FileEntry::from_path for every node,
            // which now automatically collects all flags.
            let roots = Self::build_tree_blazing_fast(&scanned);
            let tree_duration = tree_start.elapsed();

            println!("Tree built in {:.2}s", tree_duration.as_secs_f64());

            let final_data = CacheData { roots };

            println!("Writing cache file (bincode + zstd)...");
            let _ = app_bg.emit("cache_progress", "Writing cache file...");
            Self::emit_cache_build_progress(
                &app_bg,
                CacheBuildProgress {
                    phase: "writing".to_string(),
                    percent: 80,
                    message: "Writing cache file...".to_string(),
                    drives_completed: Some(total_roots),
                    total_drives: Some(total_roots.max(1)),
                    paths_found: Some(total as u64),
                    duration_seconds: Some(start.elapsed().as_secs_f64()),
                },
            );

            let write_start = std::time::Instant::now();

            match Self::write_cache_to_disk(&final_data) {
                Ok(_) => {
                    let cache_file = Self::cache_file_path();
                    let write_duration = write_start.elapsed();
                    let total_duration = start.elapsed();

                    println!("Write completed in {:.2}s", write_duration.as_secs_f64());
                    println!(
                        "TOTAL TIME: {:.2}s for {} paths",
                        total_duration.as_secs_f64(),
                        total
                    );

                    match fs::metadata(&cache_file) {
                        Ok(final_meta) => {
                            println!(
                                "Cache file: {:.2} MB",
                                final_meta.len() as f64 / 1_048_576.0
                            );

                            let _ = app_bg.emit(
                                "cache_completed",
                                format!(
                                    "Cache created in {:.1}s! {} paths",
                                    total_duration.as_secs_f64(),
                                    total
                                ),
                            );
                            service.set_cache_status(
                                0,
                                Some(format!(
                                    "Cache created! {} paths in {:.1}s",
                                    total,
                                    total_duration.as_secs_f64()
                                )),
                            );

                            service.update_global_cache(&final_data);

                            let ext_msg = "Scanning file extensions...".to_string();
                            let _ = app_bg.emit("cache_progress", ext_msg.clone());
                            Self::emit_cache_build_progress(
                                &app_bg,
                                CacheBuildProgress {
                                    phase: "extensions".to_string(),
                                    percent: 96,
                                    message: ext_msg,
                                    drives_completed: Some(total_roots),
                                    total_drives: Some(total_roots.max(1)),
                                    paths_found: Some(total as u64),
                                    duration_seconds: Some(start.elapsed().as_secs_f64()),
                                },
                            );
                            println!("🔍 Scanning file extensions from cache tree...");

                            let extensions = Self::scan_file_extensions_from_cache(&final_data);
                            println!("Found {} unique file extensions", extensions.len());

                            Self::save_extensions_to_settings(extensions);

                            let _ = app_bg.emit(
                                "extensions_ready",
                                "File extensions have been scanned and saved.",
                            );
                        }
                        Err(e) => {
                            eprintln!("Warning: Could not verify final cache file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to write cache: {}", e);
                    let _ = app_bg.emit("cache_error", format!("Failed to write cache: {}", e));
                    service.set_cache_status(0, Some(format!("Failed to write cache: {}", e)));
                }
            }
        });
    }

    // -------------------------------------------------------------------
    // Private "static" helpers (stateless — no CacheService fields used).
    // Kept on the type as associated functions so all cache logic lives
    // in one place, the same way a Java class groups static helpers
    // alongside its instance methods.
    // -------------------------------------------------------------------

    fn cache_file_path() -> PathBuf {
        PathService::new().base_path().join(CACHE_FILE_NAME)
    }

    fn cache_temp_file_path() -> PathBuf {
        PathService::new().base_path().join(CACHE_TEMP_FILE_NAME)
    }

    fn read_cache_from_disk() -> Result<CacheData, String> {
        let cache_file = Self::cache_file_path();

        if !cache_file.exists() {
            return Err("Cache file does not exist".to_string());
        }

        let compressed =
            fs::read(&cache_file).map_err(|e| format!("Failed to read cache file: {}", e))?;

        // bincode 2.x's serde Reader trait isn't implemented for zstd::Decoder,
        // so decompress to a Vec first and then decode from a byte slice.
        let decompressed = zstd::decode_all(&compressed[..])
            .map_err(|e| format!("Failed to decompress: {}", e))?;

        let (decoded, _consumed): (CacheData, usize) =
            bincode::serde::decode_from_slice(&decompressed, bincode::config::standard())
                .map_err(|e| format!("Failed to deserialize cache: {}", e))?;

        Ok(decoded)
    }

    fn write_cache_to_disk(data: &CacheData) -> Result<(), String> {
        let cache_file = Self::cache_file_path();
        let temp_file = Self::cache_temp_file_path();

        match File::create(&temp_file) {
            Ok(file) => {
                let writer = BufWriter::with_capacity(16 * 1024 * 1024, file);
                let mut encoder = zstd::Encoder::new(writer, 0)
                    .map_err(|e| format!("Failed to init encoder: {}", e))?;

                // Multi-threaded zstd uses extra CPU threads for compression (no GPU in portable zstd).
                let zstd_workers = (num_cpus::get() as u32).clamp(1, 8);
                if let Err(e) = encoder.multithread(zstd_workers) {
                    eprintln!(
                        "zstd multithread disabled ({}); continuing single-threaded",
                        e
                    );
                }

                // bincode 2.x's serde Writer trait isn't implemented for zstd::Encoder,
                // so serialize into a byte buffer first and then stream it through zstd.
                let encoded = bincode::serde::encode_to_vec(data, bincode::config::standard())
                    .map_err(|e| format!("Failed to serialize cache: {}", e))?;

                use std::io::Write;
                encoder
                    .write_all(&encoded)
                    .map_err(|e| format!("Failed to write cache bytes: {}", e))?;

                let writer = encoder
                    .finish()
                    .map_err(|e| format!("Failed to finalize compression: {}", e))?;

                let mut writer = writer;
                writer
                    .flush()
                    .map_err(|e| format!("Failed to flush cache file: {}", e))?;

                drop(writer);

                match fs::metadata(&temp_file) {
                    Ok(meta) => {
                        if meta.len() == 0 {
                            let _ = fs::remove_file(&temp_file);
                            return Err("Cache file is empty after write".to_string());
                        }
                    }
                    Err(e) => {
                        let _ = fs::remove_file(&temp_file);
                        return Err(format!("Failed to verify temp cache file: {}", e));
                    }
                }

                fs::rename(&temp_file, &cache_file)
                    .map_err(|e| format!("Failed to finalize cache file: {}", e))?;

                Ok(())
            }
            Err(e) => Err(format!("Failed to create temp cache file: {}", e)),
        }
    }

    fn remove_from_tree(nodes: &mut Vec<FileEntry>, removed_path: &str) -> usize {
        let mut removed_count = 0;

        for node in nodes.iter_mut() {
            if !Self::path_matches_or_under(&node.path, removed_path) {
                if let Some(children) = node.children.as_mut() {
                    removed_count += Self::remove_from_tree(children, removed_path);
                }
            }
        }

        nodes.retain(|node| {
            if Self::path_matches_or_under(&node.path, removed_path) {
                removed_count += 1;
                if let Some(children) = &node.children {
                    removed_count += Self::count_children_recursive(children);
                }
                false
            } else {
                true
            }
        });

        removed_count
    }

    /// Returns true when `node_path` either equals `prefix` or is a descendant of it,
    /// using a path-segment boundary so that `/foo/bar` does not match `/foo/barbaz`.
    fn path_matches_or_under(node_path: &str, prefix: &str) -> bool {
        if node_path == prefix {
            return true;
        }
        if let Some(rest) = node_path.strip_prefix(prefix) {
            return rest.starts_with('/') || rest.starts_with('\\');
        }
        false
    }

    fn count_children_recursive(children: &[FileEntry]) -> usize {
        let mut count = children.len();
        for child in children {
            if let Some(grandchildren) = &child.children {
                count += Self::count_children_recursive(grandchildren);
            }
        }
        count
    }

    fn insert_into_tree(nodes: &mut Vec<FileEntry>, new_path: &PathBuf, entry: FileEntry) -> bool {
        println!("Trying to insert: {:?}", new_path);
        println!("Into {} nodes", nodes.len());

        for node in nodes.iter_mut() {
            if node.is_dir {
                let node_path = node.ToPathbuf();
                println!("Checking node: {:?}", node_path);

                if new_path.starts_with(&node_path) {
                    println!("Path starts with this node");

                    if new_path.parent() == Some(node_path.as_path()) {
                        println!("This is the direct parent!");
                        let children = node.children.get_or_insert(Vec::new());
                        children.push(entry.clone());
                        children.sort_unstable_by(|a, b| match (a.is_dir, b.is_dir) {
                            (true, false) => std::cmp::Ordering::Less,
                            (false, true) => std::cmp::Ordering::Greater,
                            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                        });
                        return true;
                    }

                    if let Some(children) = node.children.as_mut() {
                        if Self::insert_into_tree(children, new_path, entry.clone()) {
                            return true;
                        }
                    }
                }
            }
        }

        println!("No parent found in tree");
        false
    }

    fn read_thread_settings() -> (usize, usize) {
        let settings_path = PathService::new().settings_path();

        if let Ok(content) = fs::read_to_string(&settings_path) {
            if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
                let num_cores = settings.cache.num_of_cores.max(1);
                let threads_per_core = settings.cache.thread_count_per_core.max(1);
                return (num_cores, threads_per_core);
            }
        }

        (num_cpus::get().max(1), 1)
    }

    fn cache_parallelism() -> usize {
        let logical = num_cpus::get().max(1);
        let (configured_cores, tpc) = Self::read_thread_settings();
        let configured = configured_cores.saturating_mul(tpc).max(1);
        logical.max(configured).min(256)
    }

    fn scan_directory_parallel(dir: &Path, max_depth: u32, current_depth: u32) -> Vec<PathBuf> {
        if current_depth > max_depth {
            return Vec::new();
        }

        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };

        let mut results = Vec::with_capacity(256);
        let mut subdirs = Vec::with_capacity(32);

        for entry in entries.flatten() {
            let path = entry.path();

            if Self::should_skip_path(&path) {
                continue;
            }

            results.push(path.clone());

            if path.is_dir() {
                subdirs.push(path);
            }
        }

        if subdirs.len() > 1 && current_depth < max_depth {
            let sub_results: Vec<Vec<PathBuf>> = subdirs
                .par_iter()
                .map(|subdir| Self::scan_directory_parallel(subdir, max_depth, current_depth + 1))
                .collect();

            for sub in sub_results {
                results.extend(sub);
            }
        } else {
            for subdir in subdirs {
                results.extend(Self::scan_directory_parallel(
                    &subdir,
                    max_depth,
                    current_depth + 1,
                ));
            }
        }

        results
    }

    fn should_skip_path(path: &Path) -> bool {
        if let Some(name) = path.file_name() {
            let name_str = name.to_string_lossy();

            #[cfg(target_os = "windows")]
            {
                if name_str.starts_with("$Recycle")
                    || name_str == "System Volume Information"
                    || name_str == "Windows.old"
                    || name_str == "ProgramData"
                    || name_str == "hiberfil.sys"
                    || name_str == "pagefile.sys"
                    || name_str == "DumpStack.log.tmp"
                    || name_str == "Config.Msi"
                {
                    return true;
                }
            }

            #[cfg(any(target_os = "linux", target_os = "macos"))]
            {
                if name_str.starts_with('.')
                    || name_str == "proc"
                    || name_str == "sys"
                    || name_str == "dev"
                    || name_str == "run"
                    || name_str == "tmp"
                    || name_str == "var"
                    || name_str == "snap"
                    || name_str == "lost+found"
                {
                    return true;
                }
            }

            #[cfg(target_os = "macos")]
            {
                if name_str == ".Spotlight-V100"
                    || name_str == ".Trashes"
                    || name_str == ".fseventsd"
                    || name_str == ".DocumentRevisions-V100"
                    || name_str == ".TemporaryItems"
                    || name_str == "Network"
                    || name_str == "Volumes"
                {
                    return true;
                }
            }
        }

        false
    }

    fn scan_file_extensions_from_cache(cache: &CacheData) -> Vec<SupportedFileExtension> {
        let office_extensions: HashSet<&str> = [
            "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "pdf", "rtf", "csv",
        ]
        .iter()
        .copied()
        .collect();

        let dev_extensions: HashSet<&str> = [
            "rs",
            "c",
            "cpp",
            "h",
            "hpp",
            "cs",
            "java",
            "kt",
            "swift",
            "go",
            "py",
            "rb",
            "php",
            "sh",
            "bash",
            "zsh",
            "pl",
            "lua",
            "html",
            "css",
            "js",
            "jsx",
            "ts",
            "tsx",
            "vue",
            "svelte",
            "json",
            "yaml",
            "yml",
            "toml",
            "ini",
            "cfg",
            "env",
            "md",
            "markdown",
            "sql",
            "r",
            "asm",
            "dart",
            "scala",
            "clj",
            "cljs",
            "hs",
            "elm",
            "ex",
            "exs",
            "ml",
            "mli",
            "fs",
            "fsx",
            "fsi",
            "vb",
            "vbs",
            "ps1",
            "psm1",
            "bat",
            "cmd",
            "coffee",
            "less",
            "sass",
            "scss",
            "styl",
            "graphql",
            "gql",
            "proto",
            "thrift",
            "xml",
            "xsd",
            "xslt",
            "dtd",
            "lock",
            "gradle",
            "cmake",
            "make",
            "mk",
            "ninja",
            "dockerfile",
            "gitignore",
            "gitattributes",
            "gitmodules",
            "gitconfig",
            "editorconfig",
            "prettierrc",
            "eslintrc",
            "eslintignore",
            "babelrc",
            "tsconfig",
            "jsconfig",
            "tf",
            "tfvars",
            "tfstate",
            "hcl",
            "jade",
            "pug",
            "ejs",
            "hbs",
            "mustache",
        ]
        .iter()
        .copied()
        .collect();

        let mut extension_counts: HashMap<String, i32> = HashMap::new();

        fn walk_entries(
            entries: &[FileEntry],
            counts: &mut HashMap<String, i32>,
            office: &HashSet<&str>,
            dev: &HashSet<&str>,
        ) {
            for entry in entries {
                if !entry.is_dir {
                    if let Some(ext) = Path::new(&entry.name).extension() {
                        if let Some(ext_str) = ext.to_str() {
                            let ext_lower = ext_str.to_lowercase();
                            *counts.entry(ext_lower).or_insert(0) += 1;
                        }
                    }
                }
                if let Some(children) = &entry.children {
                    walk_entries(children, counts, office, dev);
                }
            }
        }

        walk_entries(
            &cache.roots,
            &mut extension_counts,
            &office_extensions,
            &dev_extensions,
        );

        let mut result: Vec<SupportedFileExtension> = extension_counts
            .into_iter()
            .map(|(extension, count)| {
                let file_type = if office_extensions.contains(extension.as_str()) {
                    "office".to_string()
                } else if dev_extensions.contains(extension.as_str()) {
                    "dev".to_string()
                } else {
                    "other".to_string()
                };
                SupportedFileExtension {
                    extension,
                    count,
                    is_selected: false,
                    file_type,
                }
            })
            .collect();

        result.sort_by(|a, b| {
            let rank = |file_type: &str| match file_type {
                "office" => 0,
                "dev" => 1,
                _ => 2,
            };
            let rank_diff = rank(a.file_type.as_str()).cmp(&rank(b.file_type.as_str()));
            if rank_diff != std::cmp::Ordering::Equal {
                return rank_diff;
            }
            a.extension.cmp(&b.extension)
        });

        result
    }

    fn save_extensions_to_settings(extensions: Vec<SupportedFileExtension>) {
        let settings_file = PathService::new().settings_path();

        let content = match fs::read_to_string(&settings_file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Could not read settings file: {}", e);
                return;
            }
        };

        let mut settings: Settings = match serde_json::from_str(&content) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Could not parse settings file: {}", e);
                return;
            }
        };

        let mut scanned_by_key: HashMap<String, SupportedFileExtension> = extensions
            .into_iter()
            .map(|ext| (ext.extension.to_lowercase(), ext))
            .collect();

        let mut merged: Vec<SupportedFileExtension> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        for existing in settings.deep_search.supported_file_extensions.iter() {
            let key = existing.extension.to_lowercase();
            if !seen.insert(key.clone()) {
                continue;
            }

            if let Some(mut scanned) = scanned_by_key.remove(&key) {
                scanned.is_selected = existing.is_selected;
                merged.push(scanned);
            } else {
                merged.push(existing.clone());
            }
        }

        for (_, scanned) in scanned_by_key {
            let key = scanned.extension.to_lowercase();
            if seen.insert(key) {
                merged.push(scanned);
            }
        }

        merged.sort_by(|a, b| {
            let rank = |file_type: &str| match file_type {
                "office" => 0,
                "dev" => 1,
                _ => 2,
            };
            let rank_diff = rank(a.file_type.as_str()).cmp(&rank(b.file_type.as_str()));
            if rank_diff != std::cmp::Ordering::Equal {
                return rank_diff;
            }
            a.extension.cmp(&b.extension)
        });

        settings.deep_search.supported_file_extensions = merged;

        match serde_json::to_string_pretty(&settings) {
            Ok(updated) => match fs::write(&settings_file, updated) {
                Ok(_) => println!("File extensions saved to settings.json"),
                Err(e) => eprintln!("Failed to write settings file: {}", e),
            },
            Err(e) => eprintln!("Failed to serialize settings: {}", e),
        }
    }

    fn cache_build_scan_percent(completed_roots: usize, total_roots: u32) -> u8 {
        let t = total_roots.max(1) as u64;
        let c = completed_roots as u64;
        if c >= t {
            55
        } else {
            u8::try_from((c * 55) / t).unwrap_or(0).min(54)
        }
    }

    fn emit_cache_build_progress(app: &AppHandle, progress: CacheBuildProgress) {
        let _ = app.emit("cache-build-progress", progress);
    }

    fn build_tree_blazing_fast(scanned_paths: &[PathBuf]) -> Vec<FileEntry> {
        crate::core::models::cache::Cache::reset_tree_build_progress(scanned_paths.len() as u64);

        #[derive(Default)]
        struct Node {
            is_dir: bool,
            children: Vec<PathBuf>,
        }

        let mut nodes: HashMap<PathBuf, Node> = HashMap::with_capacity(scanned_paths.len());

        for path in scanned_paths {
            let is_dir = path.is_dir();

            nodes.entry(path.clone()).or_insert_with(|| Node {
                is_dir,
                children: Vec::new(),
            });

            if let Some(parent) = path.parent() {
                nodes
                    .entry(parent.to_path_buf())
                    .or_default()
                    .children
                    .push(path.clone());
            }
        }

        let has_parent: HashSet<&PathBuf> =
            nodes.values().flat_map(|n| n.children.iter()).collect();

        let mut roots: Vec<PathBuf> = nodes
            .keys()
            .filter(|p| !has_parent.contains(p))
            .cloned()
            .collect();

        roots.sort_unstable();

        fn build_entry(path: &PathBuf, nodes: &HashMap<PathBuf, Node>) -> FileEntry {
            let node = nodes.get(path);
            let is_dir = node.map(|n| n.is_dir).unwrap_or(false);

            let mut entry = FileEntry::FromPath(path, is_dir);
            crate::core::models::cache::Cache::log_cache_tree_insert(&entry.path, entry.is_dir);

            entry.children = node.and_then(|n| {
                if n.children.is_empty() {
                    None
                } else {
                    const PAR_CHILD_THRESHOLD: usize = 16;
                    let mut v: Vec<FileEntry> = if n.children.len() >= PAR_CHILD_THRESHOLD {
                        n.children
                            .par_iter()
                            .map(|c| build_entry(c, nodes))
                            .collect()
                    } else {
                        n.children.iter().map(|c| build_entry(c, nodes)).collect()
                    };

                    v.sort_unstable_by(|a, b| match (a.is_dir, b.is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                    });
                    Some(v)
                }
            });

            entry
        }

        let mut result: Vec<FileEntry> =
            roots.into_iter().map(|r| build_entry(&r, &nodes)).collect();

        result.sort_unstable_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        result
    }

    #[cfg(target_os = "windows")]
    fn get_windows_drives() -> Vec<PathBuf> {
        use windows::core::PCWSTR;
        use windows::Win32::Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives};

        const DRIVE_UNKNOWN: u32 = 0;
        const DRIVE_NO_ROOT_DIR: u32 = 1;
        const DRIVE_REMOVABLE: u32 = 2;
        const DRIVE_FIXED: u32 = 3;
        const DRIVE_REMOTE: u32 = 4;
        const DRIVE_CDROM: u32 = 5;
        const DRIVE_RAMDISK: u32 = 6;

        let mut roots = Vec::new();

        unsafe {
            let logical_drives = GetLogicalDrives();

            for i in 0..26 {
                if (logical_drives & (1 << i)) != 0 {
                    let drive_letter = (b'A' + i) as char;
                    let drive_path = format!("{}:\\", drive_letter);

                    let wide_path: Vec<u16> = drive_path
                        .encode_utf16()
                        .chain(std::iter::once(0))
                        .collect();

                    let code = GetDriveTypeW(PCWSTR(wide_path.as_ptr()));

                    if matches!(code, DRIVE_UNKNOWN | DRIVE_NO_ROOT_DIR) {
                        continue;
                    }

                    if matches!(
                        code,
                        DRIVE_REMOVABLE | DRIVE_FIXED | DRIVE_REMOTE | DRIVE_CDROM | DRIVE_RAMDISK
                    ) {
                        roots.push(PathBuf::from(drive_path));
                    }
                }
            }
        }

        roots
    }

    fn get_system_drives_impl() -> Result<Vec<String>, String> {
        #[cfg(target_os = "windows")]
        {
            let paths = Self::get_windows_drives();
            let strs = paths
                .into_iter()
                .map(|p| {
                    let entry = FileEntry::from_path(&p, true);
                    entry.path
                })
                .collect();
            Ok(strs)
        }

        #[cfg(target_os = "macos")]
        {
            Ok(vec![
                "/".to_string(),
                "/Users".to_string(),
                "/Applications".to_string(),
                "/Volumes".to_string(),
            ])
        }

        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            let mut out: Vec<String> = vec!["/".to_string(), "/home".to_string()];
            if PathBuf::from("/mnt").exists() {
                out.push("/mnt".to_string());
            }
            if PathBuf::from("/media").exists() {
                out.push("/media".to_string());
            }
            Ok(out)
        }
    }

    #[allow(dead_code)]
    fn scan_directory_fast(
        dir: &Path,
        results: &mut Vec<PathBuf>,
        max_depth: u32,
        current_depth: u32,
    ) {
        if current_depth > max_depth {
            return;
        }

        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };

        let mut local_entries = Vec::with_capacity(256);

        for entry in entries.flatten() {
            let path = entry.path();

            if Self::should_skip_path(&path) {
                continue;
            }

            local_entries.push((path.clone(), path.is_dir()));
        }

        for (path, is_dir) in local_entries {
            results.push(path.clone());

            if is_dir {
                Self::scan_directory_fast(&path, results, max_depth, current_depth + 1);
            }
        }
    }
}

// pub fn initialize_cache_directory(app_handle: AppHandle) {
//     CacheService::instance().initialize_cache_directory(app_handle);
// }

// pub fn update_cache_on_create(app: AppHandle, new_path: PathBuf) {
//     CacheService::instance().update_cache_on_create(app, new_path);
// }

// pub fn update_cache_on_remove(app: AppHandle, removed_path: PathBuf) {
//     CacheService::instance().update_cache_on_remove(app, removed_path);
// }

// #[allow(dead_code)]
// pub fn get_root_from_global_cache(normalized_root_path: &str) -> Option<FileEntry> {
//     CacheService::instance().get_root_from_global_cache(normalized_root_path)
// }

// pub fn get_cached_data() -> Result<CacheData, String> {
//     CacheService::instance().get_cached_data()
// }

// #[tauri::command]
// pub fn get_cache_status() -> Result<CacheStatus, String> {
//     CacheService::instance().get_cache_status()
// }

// #[tauri::command]
// pub fn get_system_drives() -> Result<Vec<String>, String> {
//     CacheService::get_system_drives_impl()
// }

// #[tauri::command]
// pub fn validate_cache() -> Result<String, String> {
//     CacheService::instance().validate_cache()
// }

// #[tauri::command]
// pub fn regenerate_cache(app_handle: AppHandle) -> Result<String, String> {
//     CacheService::instance().regenerate_cache(app_handle)
// }
