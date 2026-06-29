use crate::icons::*;
use crate::core::models::operations::Operation;
use dioxus::prelude::*;

pub fn file_security() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Encrypt File",
            shortcut: Some("CRTL + Shift + E"),
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Decrypt File",
            shortcut: Some("CRTL + Shift + D"),
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Scan for Malware",
            shortcut: Some("CRTL + Shift + M"),
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Calculate Hash (SHA256, MD5)",
            shortcut: Some("CRTL + Shift + H"),
        },
    ]
}
