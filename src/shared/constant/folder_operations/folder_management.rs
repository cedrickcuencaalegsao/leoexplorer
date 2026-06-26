use crate::icons::*;
use crate::shared::models::folder_operation::folder_operation::FolderOperation;
use dioxus::prelude::*;

pub fn folder_management() -> Vec<FolderOperation> {
    vec![
        FolderOperation {
            icon: rsx! {Icon{data: material_symbols_light::CreateNewFolderOutline, width:"20px", height:"20px"}},
            label: "Create New Folder Inside",
            shortcut: Some("CMD + Shift + M"),
        },
        FolderOperation {
            icon: rsx! {Icon{data: mingcute::FileNewLine, width:"20px", height:"20px"}},
            label: "Create New File Inside",
            shortcut: Some("CMD + Shift + N"),
        },
        FolderOperation {
            icon: rsx! {Icon{data: material_symbols_light::FolderZipOutlineRounded, width:"20px", height:"20px"}},
            label: "Compress (ZIP, TAR, 7z)",
            shortcut: Some("CMD + Shift + C"),
        },
        FolderOperation {
            icon: rsx! {Icon{data: material_symbols::ChipExtractionRounded, width:"20px", height:"20px"}},
            label: "Extract Here (if archive)",
            shortcut: Some("CMD + Shift + E"),
        },
        FolderOperation {
            icon: rsx! {Icon{data: tabler::FolderShare, width:"20px", height:"20px"}},
            label: "Share Folder",
            shortcut: Some("CMD + Shift + S"),
        },
    ]
}
