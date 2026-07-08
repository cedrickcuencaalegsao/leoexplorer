use crate::core::models::{cachedata::CacheData, file_entry::FileEntry};
use crate::repositories::cache::cache::CacheStatus;
use crate::repositories::cache::CacheService;
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
pub fn initialize_cache_directory(app_handle: AppHandle) {
    CacheService::instance().initialize_cache_directory(app_handle);
}

#[tauri::command]
pub fn update_cache_on_create(app: AppHandle, new_path: PathBuf) {
    CacheService::instance().update_cache_on_create(app, new_path);
}

#[tauri::command]
pub fn update_cache_on_remove(app: AppHandle, removed_path: PathBuf) {
    CacheService::instance().update_cache_on_remove(app, removed_path);
}

#[tauri::command]
pub fn get_root_from_global_cache(normalized_root_path: &str) -> Option<FileEntry> {
    CacheService::instance().get_root_from_global_cache(normalized_root_path)
}

#[tauri::command]
pub fn get_cached_data() -> Result<CacheData, String> {
    CacheService::instance().get_cached_data()
}

#[tauri::command]
pub fn get_cache_status() -> Result<CacheStatus, String> {
    CacheService::instance().get_cache_status()
}

#[tauri::command]
pub fn validate_cache() -> Result<String, String> {
    CacheService::instance().validate_cache()
}

#[tauri::command]
pub fn regenerate_cache(app_handle: AppHandle) -> Result<String, String> {
    CacheService::instance().regenerate_cache(app_handle)
}
