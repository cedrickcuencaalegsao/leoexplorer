use crate::icons::*;
use crate::core::models::operations::Operation;
use dioxus::prelude::*;

pub fn file_ai_features() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Summarize Document",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Explain File Purpose",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Generate Tags",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Detect Sensitive Information",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Suggest Better File Name",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Find Similar Files",
            shortcut: None,
        },
        Operation {
            icon: rsx! { Icon{ data: mdi::FileOutline, width: "20px", height: "20px"}},
            label: "Translate Content",
            shortcut: None,
        },
    ]
}
