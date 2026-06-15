use std::collections::HashMap;
use std::sync::Mutex;
use tauri::webview::WebviewBuilder;
use tauri::{AppHandle, Manager, WebviewUrl};

pub struct CloudViewState(pub Mutex<HashMap<String, tauri::Webview>>);

#[tauri::command]
pub async fn embed_cloud_view(
    app: AppHandle,
    label: String,
    url: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    state: tauri::State<'_, CloudViewState>,
) -> Result<(), String> {
    let mut views = state.0.lock().unwrap();

    if views.contains_key(&label) {
        return Ok(());
    }

    // ↓ get_window returns tauri::window::Window which HAS add_child
    let window = app
        .get_window("main")
        .ok_or_else(|| "main window not found".to_string())?;

    let webview = window
        .add_child(
            WebviewBuilder::new(
                &label,
                WebviewUrl::External(url.parse().map_err(|e| format!("{e}"))?),
            )
            .auto_resize(),
            tauri::LogicalPosition::new(x as f64, y as f64),
            tauri::LogicalSize::new(width as f64, height as f64),
        )
        .map_err(|e| e.to_string())?;

    views.insert(label, webview);
    Ok(())
}

#[tauri::command]
pub async fn resize_cloud_view(
    label: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    state: tauri::State<'_, CloudViewState>,
) -> Result<(), String> {
    let views = state.0.lock().unwrap();
    if let Some(view) = views.get(&label) {
        view.set_position(tauri::LogicalPosition::new(x as f64, y as f64))
            .map_err(|e| e.to_string())?;
        view.set_size(tauri::LogicalSize::new(width as f64, height as f64))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn show_cloud_view(
    label: String,
    state: tauri::State<'_, CloudViewState>,
) -> Result<(), String> {
    let views = state.0.lock().unwrap();
    if let Some(view) = views.get(&label) {
        view.show().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn hide_cloud_view(
    label: String,
    state: tauri::State<'_, CloudViewState>,
) -> Result<(), String> {
    let views = state.0.lock().unwrap();
    if let Some(view) = views.get(&label) {
        view.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn close_cloud_view(
    label: String,
    state: tauri::State<'_, CloudViewState>,
) -> Result<(), String> {
    let mut views = state.0.lock().unwrap();
    if let Some(view) = views.remove(&label) {
        view.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}
