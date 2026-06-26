use crate::icons::*;
use crate::shared::models::operations::Operation;
use dioxus::prelude::*;

pub fn folder_ai_features() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: simple_icons::Googlegemini, width: "20px", height: "20px" }},
            label: "Summarize Folder Contents",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: simple_icons::Googlegemini, width: "20px", height: "20px" }},
            label: "Detect Duplicate Files",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: simple_icons::Googlegemini, width: "20px", height: "20px" }},
            label: "Organize Files Automatically",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: simple_icons::Googlegemini, width: "20px", height: "20px" }},
            label: "Suggest Folder Cleanup",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: simple_icons::Googlegemini, width: "20px", height: "20px" }},
            label: "Explain Folder Purpose",
            shortcut: None,
        },
    ]
}
