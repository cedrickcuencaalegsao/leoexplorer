use crate::icons::*;
use crate::shared::models::folder_operation::FolderOperation;
use dioxus::prelude::*;

pub fn folder_advance_options() -> Vec<FolderOperation> {
    vec![
        FolderOperation {
            icon: rsx! {Icon { data: fa7_brands::Perbyte, width: "20px", height: "20px" }},
            label: "Calculate Folder Size",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: lucide::FolderSync, width: "20px", height: "20px" }},
            label: "Sync Folder",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: hugeicons::Encrypt, width: "20px", height: "20px" }},
            label: "Encrypt Folder",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: ix::Tree, width: "20px", height: "20px" }},
            label: "Generate Folder Tree",
            shortcut: None,
        },
    ]
}
