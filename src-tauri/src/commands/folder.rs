use crate::core::models::file_entry::FileEntry;
use crate::repositories::folder::FolderService;
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
pub fn open_folder(path: PathBuf) -> Result<Vec<FileEntry>, String> {
    FolderService::new().open_folder(&path)
}

#[tauri::command]
pub fn get_folder_children(path: PathBuf) -> Result<Vec<FileEntry>, String> {
    FolderService::new().get_folder_children(&path)
}

#[tauri::command]
pub fn create_new_folder(
    app_handle: AppHandle,
    path: String,
    name: String,
) -> Result<String, String> {
    FolderService::new().create_new_folder(&app_handle, &path, &name)
}

#[tauri::command]
pub fn rename_folder(
    app_handle: AppHandle,
    path: String,
    new_name: String,
) -> Result<String, String> {
    FolderService::new().rename_folder(app_handle, &path, &new_name)
}
