use crate::icons::*;
use crate::core::models::operations::Operation;
use dioxus::prelude::*;

#[allow(dead_code)]
pub fn audio() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Play",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Edit Metadata",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Convert Format",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Normalize Volume",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Generate Transcript",
            shortcut: None,
        },
    ]
}
