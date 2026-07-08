use crate::commands::{
    cache::{
        get_cache_status, get_cached_data, get_root_from_global_cache, initialize_cache_directory,
        regenerate_cache, update_cache_on_create, update_cache_on_remove, validate_cache,
    },
    cloud_view::{
        close_cloud_view, embed_cloud_view, hide_cloud_view, resize_cloud_view, show_cloud_view,
    },
    drive::{init_cache, open_roots},
    folder::{create_new_folder, get_folder_children, open_folder, rename_folder},
};
use crate::repositories::cloud_view_manager::CloudViewManager;
use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

mod commands;
mod core;
mod repositories;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(CloudViewManager::new())
        .invoke_handler(tauri::generate_handler![
            // Cloude views api
            embed_cloud_view,
            resize_cloud_view,
            show_cloud_view,
            hide_cloud_view,
            close_cloud_view,
            // Folder api
            create_new_folder,
            get_folder_children,
            open_folder,
            rename_folder,
            // Cache api,
            get_cache_status,
            get_cached_data,
            get_root_from_global_cache,
            initialize_cache_directory,
            regenerate_cache,
            update_cache_on_create,
            update_cache_on_remove,
            validate_cache,
            // Drive api.
            init_cache,
            open_roots
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
