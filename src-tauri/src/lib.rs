use crate::commands::cloud_view::{
    close_cloud_view, embed_cloud_view, hide_cloud_view, resize_cloud_view, show_cloud_view,
    CloudViewState,
};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

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
        .setup(|app| {
            let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Leo Explorer")
                .inner_size(1200.00, 820.00);

            #[cfg(target_os = "macos")]
            {
                builder = builder
                    .title_bar_style(TitleBarStyle::Overlay)
                    .hidden_title(true)
                    .traffic_light_position(tauri::LogicalPosition::new(12.0, 16.0));
            }

            #[cfg(not(target_os = "macos"))]
            {
                builder = builder.decorations(false);
            }

            builder.build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
