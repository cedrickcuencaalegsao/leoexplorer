use crate::icons::*;
use crate::shared::models::folder_operation::FolderOperation;
use dioxus::prelude::*;

pub fn folder_basic_operations() -> Vec<FolderOperation> {
    vec![
        FolderOperation {
            icon: rsx! { Icon { data: material_symbols::FolderOpenOutline, width: "20", height: "20" } },
            label: "Open Folder",
            shortcut: Some("ENTER"),
        },
        FolderOperation {
            icon: rsx! { Icon { data: fluent::WindowNew20Regular, width: "20", height: "20"  } },
            label: "Open Folder in new tab",
            shortcut: Some("CMD + ENTER"),
        },
        FolderOperation {
            icon: rsx! { Icon { data: radix_icons::OpenInNewWindow, width: "20", height: "20" } },
            label: "Open Folder in new window",
            shortcut: Some("CMD + SHIFT + ENTER"),
        },
        FolderOperation {
            icon: rsx! { Icon { data: gg::Rename, width: "20", height: "20"  } },
            label: "Rename",
            shortcut: Some("F2"),
        },
        FolderOperation {
            icon: rsx! { Icon { data: material_symbols::DeleteOutline , width: "20", height: "20" } },
            label: "Delete",
            shortcut: Some("DELETE"),
        },
        FolderOperation {
            icon: rsx! { Icon { data: boxicons::Cut, width: "20", height: "20"  } },
            label: "Cut",
            shortcut: Some("CMD + X"),
        },
        FolderOperation {
            icon: rsx! { Icon { data: cuida::CopyOutline, width: "20", height: "20"  } },
            label: "Copy",
            shortcut: Some("CMD + C"),
        },
        FolderOperation {
            icon: rsx! { Icon { data: ion::DuplicateOutline, width: "20", height: "20"  } },
            label: "Duplicate",
            shortcut: Some("CMD + D"),
        },
        FolderOperation {
            icon: rsx! { Icon { data: streamline_ultimate::MoveToBottom, width: "20", height: "20"  } },
            label: "Move to...",
            shortcut: Some("CMD + M"),
        },
        FolderOperation {
            icon: rsx! { Icon { data: clarity::CopyToClipboardLine, width: "20", height: "20"  } },
            label: "Copy to...",
            shortcut: Some("CMD + SHIFT + C"),
        },
    ]
}
