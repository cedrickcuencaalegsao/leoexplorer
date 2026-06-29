use crate::icons::*;
use crate::core::models::operations::Operation;
use dioxus::prelude::*;

#[allow(dead_code)]
pub fn videos() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Preview",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Trim",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Compress",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Convert Format",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Extract Audio",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Generate Subtitle",
            shortcut: None,
        },
    ]
}
