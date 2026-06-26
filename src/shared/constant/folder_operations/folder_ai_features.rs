use crate::icons::*;
use crate::shared::models::folder_operation::FolderOperation;
use dioxus::prelude::*;

pub fn folder_ai_features() -> Vec<FolderOperation> {
    vec![
        FolderOperation {
            icon: rsx! {Icon { data: simple_icons::Googlegemini, width: "20px", height: "20px" }},
            label: "Summarize Folder Contents",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: simple_icons::Googlegemini, width: "20px", height: "20px" }},
            label: "Detect Duplicate Files",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: simple_icons::Googlegemini, width: "20px", height: "20px" }},
            label: "Organize Files Automatically",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: simple_icons::Googlegemini, width: "20px", height: "20px" }},
            label: "Suggest Folder Cleanup",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: simple_icons::Googlegemini, width: "20px", height: "20px" }},
            label: "Explain Folder Purpose",
            shortcut: None,
        },
    ]
}
