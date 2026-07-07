use crate::icons::*;
use crate::core::models::operations::Operation;
use dioxus::prelude::*;

pub fn file_sharing() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Share File",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Generate Link",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Copy File Path",
            shortcut: None,
        },
    ]
}
