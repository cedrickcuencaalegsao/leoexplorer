use crate::icons::*;
use crate::core::models::operations::Operation;
use dioxus::prelude::*;

#[allow(dead_code)]
pub fn dev_files() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Open",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Open in Editor",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Run",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Debug",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Format Code",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Generate Documentation",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "AI Explain Code",
            shortcut: None,
        },
    ]
}
