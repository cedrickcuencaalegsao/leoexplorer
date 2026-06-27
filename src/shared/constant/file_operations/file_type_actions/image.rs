use crate::icons::*;
use crate::shared::models::operations::Operation;
use dioxus::prelude::*;

#[allow(dead_code)]
pub fn image() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Preview",
            shortcut: Some("CMD + O"),
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Edit",
            shortcut: Some("CMD + O"),
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Rotate",
            shortcut: Some("CMD + O"),
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Resize",
            shortcut: Some("CMD + O"),
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Convert Format (PNG → JPG)",
            shortcut: Some("CMD + O"),
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "Remove Background",
            shortcut: Some("CMD + O"),
        },
        Operation {
            icon: rsx! {Icon { data: mdi::FileOutline, width: "20px", height: "20px" }},
            label: "AI Image Description",
            shortcut: Some("CMD + O"),
        },
    ]
}
