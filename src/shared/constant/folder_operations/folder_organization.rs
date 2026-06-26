use crate::icons::*;
use crate::shared::models::folder_operation::FolderOperation;
use dioxus::prelude::*;

pub fn get_organization_operations() -> Vec<FolderOperation> {
    vec![
        FolderOperation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Organize by Name",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Organize by Size",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Organize by Date",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Organize by Type",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Search Within Folder",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Sort Contents",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Filter Contents",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon{data: streamline_ultimate::Hierarchy5Organize, width:"20px", height:"20px"}},
            label: "Group by Type/Date/Size",
            shortcut: None,
        },
    ]
}
