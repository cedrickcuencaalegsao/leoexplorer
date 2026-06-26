use crate::icons::*;
use crate::shared::models::folder_operation::folder_operation::FolderOperation;
use dioxus::prelude::*;

pub fn navigation() -> Vec<FolderOperation> {
    vec![
        FolderOperation {
            icon: rsx! {Icon { data: tabler::Heart, width:"20px", height:"20px" }},
            label: "Add to Favorites",
            shortcut: Some("CMD + F"),
        },
        FolderOperation {
            icon: rsx! {Icon { data: mynaui::Pin, width:"20px", height:"20px" }},
            label: "Pin to Quick Access",
            shortcut: Some("CMD + Q"),
        },
        FolderOperation {
            icon: rsx! {Icon { data: solar::FolderPathConnectOutline, width:"20px", height:"20px" }},
            label: "Copy Folder Path",
            shortcut: Some("CMD + P"),
        },
        FolderOperation {
            icon: rsx! {Icon { data: octicon::FileDirectoryOpenFill24, width:"20px", height:"20px" }},
            label: "Open Containing Directory (if shortcut)",
            shortcut: None,
        },
    ]
}
