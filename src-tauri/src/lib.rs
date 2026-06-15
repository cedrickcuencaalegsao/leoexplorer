use crate::commands::cloud_view::{
    close_cloud_view, embed_cloud_view, hide_cloud_view, resize_cloud_view, show_cloud_view,
    CloudViewState,
};
use std::collections::HashMap;
use std::sync::Mutex;

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(CloudViewState(Mutex::new(HashMap::new())))
        .invoke_handler(tauri::generate_handler![
            embed_cloud_view,
            resize_cloud_view,
            show_cloud_view,
            hide_cloud_view,
            close_cloud_view,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
 