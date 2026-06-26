use crate::icons::*;
use crate::shared::models::folder_operation::FolderOperation;
use dioxus::prelude::*;

pub fn folder_information() -> Vec<FolderOperation> {
    vec![
        FolderOperation {
            icon: rsx! {Icon { data: qlementine_icons::Properties16, width: "20px", height: "20px" }},
            label: "Properties",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: fa7_brands::Perbyte, width: "20px", height: "20px" }},
            label: "Size",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: game_icons::Files, width: "20px", height: "20px" }},
            label: "Number of files",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: lets_icons::FoldersLine, width: "20px", height: "20px" }},
            label: "Number of subfolders",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: eos_icons::SubscriptionsCreatedOutlined, width: "20px", height: "20px" }},
            label: "Creation date",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: eos_icons::ModifiedDateOutlined, width: "20px", height: "20px" }},
            label: "Modified date",
            shortcut: None,
        },
        FolderOperation {
            icon: rsx! {Icon { data: icon_park_outline::Permissions, width: "20px", height: "20px" }},
            label: "Permissions",
            shortcut: None,
        },
    ]
}
