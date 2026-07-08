use crate::core::models::file_entry::FileEntry;
use crate::repositories::drive::DriveService;
use std::path::Path;

#[tauri::command]
pub fn open_roots(path: &Path) -> Result<Vec<FileEntry>, String> {
    DriveService::new().open_roots(path)
}

#[tauri::command]
pub fn init_cache() {
    DriveService::init_cache();
}
