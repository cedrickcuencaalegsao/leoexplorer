use crate::icons::*;
use crate::shared::models::operations::Operation;
use dioxus::prelude::*;

#[allow(dead_code)]
pub fn archive() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Open",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Extract",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Extract Here",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Extract To...",
            shortcut: None,
        },
    ]
}
