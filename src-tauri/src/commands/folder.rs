use crate::core::models::file_entry::FileEntry;
use crate::repositories::folder::FolderService;
use serde_json::{json, Value};
use std::path::PathBuf;
use tauri::AppHandle;

#[tauri::command]
pub fn get_default_start_path() -> String {
    FolderService::default_start_path()
        .to_string_lossy()
        .to_string()
}

#[tauri::command]
pub fn open_folder(path: PathBuf, view_mode: String) -> Result<Vec<Value>, String> {
    let entries = FolderService::new().open_folder(&path)?;

    let items: Vec<Value> = entries
        .into_iter()
        .map(|e| {
            let item_type = if e.is_dir {
                json!({ "Folder": null })
            } else {
                json!({ "File": e.file_type.clone().unwrap_or_else(|| "Unknown".into()) })
            };

            let flag = derive_flag(&e.flag);

            json!({
                "name": e.name,
                "item_type": item_type,
                "date_created": e.date_created.unwrap_or_default(),
                "date_modified": e.date_modified.unwrap_or_default(),
                "flag": flag,
                "path": e.path,
                "is_dir": e.is_dir,
                "view_mode": view_mode,
            })
        })
        .collect();

    for item in &items {
        println!(
            "{} | dir: {} | type: {}",
            item.get("name").and_then(|v| v.as_str()).unwrap_or("?"),
            item.get("is_dir")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            item.get("item_type")
                .map(|v| v.to_string())
                .unwrap_or_default(),
        );
    }

    Ok(items)
}

fn derive_flag(flags: &[crate::core::enums::flag::Flag]) -> Option<String> {
    use crate::core::enums::{
        flag::Flag, fs_attribute::FsAttribute, item_permissions::ItemPermissions,
    };

    let is_hidden = flags
        .iter()
        .any(|f| matches!(f, Flag::Attribute(FsAttribute::Hidden)));
    if is_hidden {
        return Some("HIDDEN".to_string());
    }

    let has_write = flags.iter().any(|f| {
        matches!(
            f,
            Flag::ItemPermission(ItemPermissions::Write | ItemPermissions::FullControl)
        )
    });
    if !has_write {
        return Some("LOCKED".to_string());
    }

    None
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
