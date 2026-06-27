use crate::repositories::cloud_view_manager::CloudViewManager;
use tauri::AppHandle;

#[tauri::command]
pub async fn embed_cloud_view(
    app: AppHandle,
    label: String,
    url: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    manager: tauri::State<'_, CloudViewManager>,
) -> Result<(), String> {
    manager.embed(&app, label, url, x, y, width, height)
}

#[tauri::command]
pub async fn resize_cloud_view(
    label: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    manager: tauri::State<'_, CloudViewManager>,
) -> Result<(), String> {
    manager.resize(&label, x, y, width, height)
}

#[tauri::command]
pub async fn show_cloud_view(
    label: String,
    manager: tauri::State<'_, CloudViewManager>,
) -> Result<(), String> {
    manager.show(&label)
}

#[tauri::command]
pub async fn hide_cloud_view(
    label: String,
    manager: tauri::State<'_, CloudViewManager>,
) -> Result<(), String> {
    manager.hide(&label)
}

#[tauri::command]
pub async fn close_cloud_view(
    label: String,
    manager: tauri::State<'_, CloudViewManager>,
) -> Result<(), String> {
    manager.close(&label)
}
