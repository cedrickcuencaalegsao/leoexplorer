use crate::core::models::cache::cache;
use crate::core::models::file_entry::FileEntry;
use crate::repositories::cache::CacheService;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use tauri::AppHandle;

pub struct FolderService;

impl Default for FolderService {
    fn default() -> Self {
        FolderService
    }
}

impl FolderService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_folder(&self, path: &Path) -> Result<Vec<FileEntry>, String> {
        if !path.exists() {
            return Err("Path does not exist".to_string());
        }
        Ok(self.read_directory_shallow_fast(path))
    }

    pub fn get_folder_children(&self, path: &Path) -> Result<Vec<FileEntry>, String> {
        self.open_folder(path)
    }

    pub fn create_new_folder(
        &self,
        app_handle: &AppHandle,
        path: &str,
        name: &str,
    ) -> Result<String, String> {
        println!("creating {} @ {}", name, path);

        let mut folder_path = PathBuf::from(path);
        folder_path.push(name);

        if folder_path.exists() {
            return Err("Folder already exists".to_string());
        };

        match fs::create_dir_all(&folder_path) {
            Ok(_) => {
                let success_message =
                    format!("Folder created successfully: {}", folder_path.display());
                println!("{}", success_message);

                let app_handle_clone = app_handle.clone();
                let folder_path_clone = folder_path.clone();

                thread::spawn(move || {
                    // mag update ni dri sa cache nato
                    // cache::update_cache_on_create(app_handle_clone, folder_path_clone);
                    CacheService::instance()
                        .update_cache_on_create(app_handle_clone, folder_path_clone);
                });

                Ok(success_message)
            }
            Err(e) => {
                let error_message = format!(
                    "Failed to create folder {} at {}: {}",
                    name,
                    folder_path.display(),
                    e
                );
                Err(error_message)
            }
        }
    }

    pub fn rename_folder(
        &self,
        app_handle: AppHandle,
        path: &str,
        new_name: &str,
    ) -> Result<String, String> {
        println!("Renaming folder: {} -> {}", path, new_name);
        let old_path = Path::new(path);

        if !old_path.exists() {
            return Err(format!("Path does not exist: {}", path));
        }
        if !old_path.is_dir() {
            return Err(format!("Path is not a folder: {}", path));
        }

        let parent = old_path
            .parent()
            .ok_or_else(|| "Cannot rename the root directory".to_string())?;

        let new_name = new_name.trim();
        if new_name.is_empty() {
            return Err("New folder name cannot be empty".to_string());
        }

        if new_name.contains('/') || new_name.contains('\\') || new_name.contains("..") {
            return Err("Invalid folder name: cannot contain path separators or '..'".to_string());
        }

        let new_path = parent.join(new_name);

        if new_path.exists() {
            return Err(format!(
                "A file or folder named '{}' already exists in the same location",
                new_name
            ));
        }

        fs::rename(old_path, &new_path).map_err(|e| format!("Failed to rename folder: {}", e))?;

        let old_path_buf = PathBuf::from(path);
        let new_path_buf = new_path.clone();
        let app_handle_clone = app_handle.clone();
        thread::spawn(move || {
            // cache::update_cache_on_remove(app_handle_clone.clone(), old_path_buf);
            // cache::update_cache_on_create(app_handle_clone, new_path_buf);
            CacheService::instance().update_cache_on_remove(app_handle_clone.clone(), old_path_buf);
            CacheService::instance().update_cache_on_create(app_handle_clone, new_path_buf);
        });

        Ok(format!("Successfully renamed folder to: {}", new_name))
    }

    fn read_directory_shallow_fast(&self, path: &Path) -> Vec<FileEntry> {
        let read_dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(_) => return vec![],
        };

        let mut entries = Vec::with_capacity(50);

        for entry in read_dir.flatten() {
            let entry_path = entry.path();
            let is_dir = match entry.metadata() {
                Ok(meta) => meta.is_dir(),
                Err(_) => continue,
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
            _ => a.name.cmp(&b.name),
        });

        entries
    }
}
