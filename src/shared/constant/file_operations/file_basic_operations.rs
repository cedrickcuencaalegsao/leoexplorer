use crate::icons::*;
use crate::shared::models::operations::Operation;
use dioxus::prelude::*;

pub fn file_basic_operations() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Open",
            shortcut: Some("CMD + O"),
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Open with...",
            shortcut: Some("CMD + Shift + O"),
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Open in Terminal",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Open in Finder", // Finder for macOS, Explorer for Windows, and what for Linux
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Rename",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Delete",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Cut",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Copy",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Paste",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Duplicate",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Copy Path",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Move to...",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Copy to...",
            shortcut: None,
        },
    ]
}
