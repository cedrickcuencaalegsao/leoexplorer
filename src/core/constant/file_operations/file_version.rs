use crate::icons::*;
use crate::core::models::operations::Operation;
use dioxus::prelude::*;

pub fn file_version() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "View History",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Restore Previous Version",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Compare Versions",
            shortcut: None,
        },
    ]
}
