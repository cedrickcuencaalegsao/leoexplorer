use crate::icons::*;
use crate::shared::models::operations::operation::Operation;
use dioxus::prelude::*;

pub fn folder_management() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon{data: material_symbols_light::CreateNewFolderOutline, width:"20px", height:"20px"}},
            label: "Create New Folder Inside",
            shortcut: Some("CMD + Shift + M"),
        },
        Operation {
            icon: rsx! {Icon{data: mingcute::FileNewLine, width:"20px", height:"20px"}},
            label: "Create New File Inside",
            shortcut: Some("CMD + Shift + N"),
        },
        Operation {
            icon: rsx! {Icon{data: material_symbols_light::FolderZipOutlineRounded, width:"20px", height:"20px"}},
            label: "Compress (ZIP, TAR, 7z)",
            shortcut: Some("CMD + Shift + C"),
        },
        Operation {
            icon: rsx! {Icon{data: material_symbols::ChipExtractionRounded, width:"20px", height:"20px"}},
            label: "Extract Here (if archive)",
            shortcut: Some("CMD + Shift + E"),
        },
        Operation {
            icon: rsx! {Icon{data: tabler::FolderShare, width:"20px", height:"20px"}},
            label: "Share Folder",
            shortcut: Some("CMD + Shift + S"),
        },
    ]
}
