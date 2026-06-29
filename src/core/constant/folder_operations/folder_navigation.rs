use crate::icons::*;
use crate::core::models::operations::operation::Operation;
use dioxus::prelude::*;

pub fn navigation() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width:"20px", height:"20px" }},
            label: "Add to Favorites",
            shortcut: Some("CMD + F"),
        },
        Operation {
            icon: rsx! {Icon { data: mynaui::Pin, width:"20px", height:"20px" }},
            label: "Pin to Quick Access",
            shortcut: Some("CMD + Q"),
        },
        Operation {
            icon: rsx! {Icon { data: solar::FolderPathConnectOutline, width:"20px", height:"20px" }},
            label: "Copy Folder Path",
            shortcut: Some("CMD + P"),
        },
        Operation {
            icon: rsx! {Icon { data: octicon::FileDirectoryOpenFill24, width:"20px", height:"20px" }},
            label: "Open Containing Directory (if shortcut)",
            shortcut: None,
        },
    ]
}
