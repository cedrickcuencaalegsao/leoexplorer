use crate::icons::*;
use crate::shared::models::operations::Operation;
use dioxus::prelude::*;

pub fn folder_information() -> Vec<Operation> {
    vec![
        Operation {
            icon: rsx! {Icon { data: qlementine_icons::Properties16, width: "20px", height: "20px" }},
            label: "Properties",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: fa7_brands::Perbyte, width: "20px", height: "20px" }},
            label: "Size",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: game_icons::Files, width: "20px", height: "20px" }},
            label: "Number of files",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: lets_icons::FoldersLine, width: "20px", height: "20px" }},
            label: "Number of subfolders",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: eos_icons::SubscriptionsCreatedOutlined, width: "20px", height: "20px" }},
            label: "Creation date",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: eos_icons::ModifiedDateOutlined, width: "20px", height: "20px" }},
            label: "Modified date",
            shortcut: None,
        },
        Operation {
            icon: rsx! {Icon { data: icon_park_outline::Permissions, width: "20px", height: "20px" }},
            label: "Permissions",
            shortcut: None,
        },
    ]
}
