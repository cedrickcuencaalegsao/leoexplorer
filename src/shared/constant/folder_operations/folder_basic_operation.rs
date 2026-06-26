use crate::icons::*;
use crate::shared::models::operations::Operation;
use dioxus::prelude::*;

pub fn folder_basic_operations() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! { Icon { data: material_symbols::FolderOpenOutline, width: "20", height: "20" } },
            label: "Open Folder",
            shortcut: Some("ENTER"),
        },
        Operation {
            icon: rsx! { Icon { data: fluent::WindowNew20Regular, width: "20", height: "20"  } },
            label: "Open Folder in new tab",
            shortcut: Some("CMD + ENTER"),
        },
        Operation {
            icon: rsx! { Icon { data: radix_icons::OpenInNewWindow, width: "20", height: "20" } },
            label: "Open Folder in new window",
            shortcut: Some("CMD + SHIFT + ENTER"),
        },
        Operation {
            icon: rsx! { Icon { data: gg::Rename, width: "20", height: "20"  } },
            label: "Rename",
            shortcut: Some("F2"),
        },
        Operation {
            icon: rsx! { Icon { data: material_symbols::DeleteOutline , width: "20", height: "20" } },
            label: "Delete",
            shortcut: Some("DELETE"),
        },
        Operation {
            icon: rsx! { Icon { data: boxicons::Cut, width: "20", height: "20"  } },
            label: "Cut",
            shortcut: Some("CMD + X"),
        },
        Operation {
            icon: rsx! { Icon { data: cuida::CopyOutline, width: "20", height: "20"  } },
            label: "Copy",
            shortcut: Some("CMD + C"),
        },
        Operation {
            icon: rsx! { Icon { data: ion::DuplicateOutline, width: "20", height: "20"  } },
            label: "Duplicate",
            shortcut: Some("CMD + D"),
        },
        Operation {
            icon: rsx! { Icon { data: streamline_ultimate::MoveToBottom, width: "20", height: "20"  } },
            label: "Move to...",
            shortcut: Some("CMD + M"),
        },
        Operation {
            icon: rsx! { Icon { data: clarity::CopyToClipboardLine, width: "20", height: "20"  } },
            label: "Copy to...",
            shortcut: Some("CMD + SHIFT + C"),
        },
    ]
}
