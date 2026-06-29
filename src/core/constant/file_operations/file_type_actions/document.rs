use crate::icons::*;
use crate::core::models::operations::Operation;
use dioxus::prelude::*;

#[allow(dead_code)]
pub fn document() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Preview",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Edit",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Print",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Convert to PDF",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Export",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "AI Summary",
            shortcut: None,
        },
    ]
}
