use crate::core::constant::CACHE_FILE_NAME;
use crate::core::models::file_entry::FileEntry;
use crate::core::models::{appcachestate::AppCacheState, inmemorycache::InMemoryCache};
use crate::repositories::path::PathService;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct DriveService;

impl Default for DriveService {
    fn default() -> Self {
        DriveService
    }
}

impl DriveService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_cache() -> AppCacheState {
        let cache_path = Self::new().cache_file_path();

        if cache_path.exists() {
            println!("Cache file found at startup: {:?}", cache_path);
        } else {
            println!("No cache file found at startup: {:?}", cache_path);
        }

        AppCacheState {
            data: Mutex::new(InMemoryCache::new()),
        }
    }

    pub fn open_roots(&self, path: &Path) -> Result<Vec<FileEntry>, String> {
        let path_str = path.to_str().unwrap_or("");
        let formatted_path = self.format_path(&path_str);
        let entries = self.read_drive_contents(&formatted_path);
        Ok(entries)
    }

    fn cache_file_path(&self) -> PathBuf {
        PathService::new().base_path().join(CACHE_FILE_NAME)
    }

    fn format_path(&self, path_or_letter: &str) -> PathBuf {
        let s = path_or_letter.trim();

        if cfg!(target_os = "windows") {
            let letter = s.trim_end_matches(':');
            if letter.len() == 1
                && letter
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_alphabetic())
                    .unwrap_or(false)
            {
                PathBuf::from(format!("{}:\\", letter.to_uppercase()))
            } else if s.contains('\\') || s.contains('/') {
                PathBuf::from(s)
            } else {
                PathBuf::from(format!("{}\\", letter.to_uppercase()))
            }
        } else {
            PathBuf::from(if s.is_empty() { "/" } else { s })
        }
    }

    fn read_drive_contents(&self, path: &Path) -> Vec<FileEntry> {
        let mut entries = Vec::with_capacity(100);
        let read_dir = match std::fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_) => return entries,
        };

        for entry in read_dir.flatten() {
            let entry_path = entry.path();
            let is_dir = match entry.metadata() {
                Ok(m) => m.is_dir(),
                Err(_) => false,
            };
            let mut file_entry = FileEntry::FromPath(&entry_path, is_dir);
            if is_dir {
                file_entry.children = Some(Vec::new());
            }
            entries.push(file_entry);
        }

        entries.sort_unstable_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        entries
    }
}
