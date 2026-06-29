use crate::icons::*;
use crate::core::models::operations::Operation;
use dioxus::prelude::*;

pub fn file_informations() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Name",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Extension",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Size",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Created Date",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Modified Date",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Permissions",
            shortcut: None,
        },
    ]
}
