use std::collections::HashMap;
use std::sync::Mutex;
use tauri::webview::WebviewBuilder;
use tauri::{AppHandle, Manager, WebviewUrl};

pub struct CloudViewManager {
    views: Mutex<HashMap<String, tauri::Webview>>,
}

impl Default for CloudViewManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudViewManager {
    pub fn new() -> Self {
        Self {
            views: Mutex::new(HashMap::new()),
        }
    }

    pub fn embed(
        &self,
        app: &AppHandle,
        label: String,
        url: String,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let mut views = self.views.lock().unwrap();

        if views.contains_key(&label) {
            return Ok(());
        }

        let window = app
            .get_window("main")
            .ok_or_else(|| "Main window not found".to_string())?;

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

    pub fn resize(
        &self,
        label: &str,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        let views = self.views.lock().unwrap();

        if let Some(view) = views.get(label) {
            view.set_position(tauri::LogicalPosition::new(x as f64, y as f64))
                .map_err(|e| e.to_string())?;
            view.set_size(tauri::LogicalSize::new(width as f64, height as f64))
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn show(&self, label: &str) -> Result<(), String> {
        let views = self.views.lock().unwrap();

        if let Some(view) = views.get(label) {
            view.show().map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn hide(&self, label: &str) -> Result<(), String> {
        let views = self.views.lock().unwrap();

        if let Some(view) = views.get(label) {
            view.hide().map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub fn close(&self, label: &str) -> Result<(), String> {
        let views = self.views.lock().unwrap();

        if let Some(view) = views.get(label) {
            view.close().map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
