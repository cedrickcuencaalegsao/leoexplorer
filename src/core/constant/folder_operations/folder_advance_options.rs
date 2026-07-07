use crate::icons::*;
use crate::core::models::operations::Operation;
use dioxus::prelude::*;

pub fn folder_advance_options() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: fa7_brands::Perbyte, width: "20px", height: "20px" }},
            label: "Calculate Folder Size",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: lucide::FolderSync, width: "20px", height: "20px" }},
            label: "Sync Folder",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: hugeicons::Encrypt, width: "20px", height: "20px" }},
            label: "Encrypt Folder",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: ix::Tree, width: "20px", height: "20px" }},
            label: "Generate Folder Tree",
            shortcut: None,
        },
    ]
}
