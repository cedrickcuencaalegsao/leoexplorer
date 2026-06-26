use crate::icons::*;
use crate::shared::models::operations::Operation;
use dioxus::prelude::*;

pub fn get_organization_operations() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Organize by Name",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Organize by Size",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Organize by Date",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Organize by Type",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Search Within Folder",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Sort Contents",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Filter Contents",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Group by Type/Date/Size",
            shortcut: None,
        },
    ]
}
