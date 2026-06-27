use crate::icons::*;
use crate::shared::models::operations::Operation;
use dioxus::prelude::*;

pub fn file_basic_operations() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Open",
            shortcut: Some("CMD + O"),
        },
        Operation {
            icon: rsx! {Icon { data: material_symbols::FileOpenOutlineRounded, width: "20px", height: "20px" }},
            label: "Open with...",
            shortcut: Some("CMD + Shift + O"),
        },
        Operation {
            icon: rsx! {Icon { data: material_symbols::TerminalRounded, width: "20px", height: "20px" }},
            label: "Open in Terminal",
            shortcut: Some("CMD + J"),
        },
        Operation {
            icon: rsx! {Icon { data: iconoir::Finder, width: "20px", height: "20px" }},
            label: "Open in Finder", // Finder for macOS, Explorer for Windows, and what for Linux
            shortcut: Some("CMD + F"),
        },
        Operation {
            icon: rsx! {Icon { data: gg::Rename, width: "20px", height: "20px" }},
            label: "Rename",
            shortcut: Some("F2"),
        },
        Operation {
            icon: rsx! {Icon { data: material_symbols::DeleteOutline, width: "20px", height: "20px" }},
            label: "Delete",
            shortcut: Some("CMD + D"),
        },
        Operation {
            icon: rsx! {Icon { data: boxicons::Cut, width: "20px", height: "20px" }},
            label: "Cut",
            shortcut: Some("CMD + X"),
        },
        Operation {
            icon: rsx! {Icon { data: cuida::CopyOutline, width: "20px", height: "20px" }},
            label: "Copy",
            shortcut: Some("CMD + C"),
        },
        Operation {
            icon: rsx! {Icon { data: fa6_regular::Paste, width: "20px", height: "20px" }},
            label: "Paste",
            shortcut: Some("CMD + P"),
        },
        Operation {
            icon: rsx! {Icon { data: ion::DuplicateOutline, width: "20px", height: "20px" }},
            label: "Duplicate",
            shortcut: Some("CMD + Shift + D"),
        },
        Operation {
            icon: rsx! {Icon { data: tabler::Heart, width: "20px", height: "20px" }},
            label: "Copy Path",
            shortcut: Some("CMD + Shift + P"),
        },
        Operation {
            icon: rsx! {Icon { data: streamline_ultimate::MoveToBottom, width: "20px", height: "20px" }},
            label: "Move to...",
            shortcut: Some("CMD + Shift + M"),
        },
        Operation {
            icon: rsx! {Icon { data: clarity::CopyToClipboardLine, width: "20px", height: "20px" }},
            label: "Copy to...",
            shortcut: Some("CMD + Shift + C"),
        },
    ]
}
