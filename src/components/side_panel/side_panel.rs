#![allow(non_snake_case)]
use crate::components::{sp_cloud::SpCloud, sp_drive::SpDrive, sp_items::SpItems, sp_menu::SpMenu};
use crate::icons::*;
use crate::shared::{
    constant::constant::{FOLDER_COLOR, GREY},
    design::design::side_panel_style,
};
use dioxus::prelude::*;

pub const APP_ICON: Asset = asset!("/assets/icons/app-icon.png");

#[component]
pub fn SidePanel() -> Element {
    let expand_sp_menu = use_signal(|| Vec::<String>::new());
    let sp_menu: Vec<(&str, Element)> = vec![
        (
            "Home",
            rsx! {Icon {data: material_symbols::HomeOutlineRounded, width: "20", height: "20"} },
        ),
        (
            "Desktop",
            rsx! {Icon {data: uil::Desktop, width: "20", height: "20"} },
        ),
        (
            "Documents",
            rsx! {Icon {data: mingcute::DocumentsLine, width: "20", height: "20"} },
        ),
        (
            "Downloads",
            rsx! {Icon {data: material_symbols::DownloadRounded, width: "20", height: "20"} },
        ),
        (
            "Music",
            rsx! {Icon{ data: mdi::Music, width: "20", height: "20" } },
        ),
        (
            "Pictures",
            rsx! {Icon { data:proicons::Photo, width: "20", height: "20" } },
        ),
        (
            "Movies",
            rsx! {Icon {data: bxs::Videos, width: "20", height: "20"} },
        ),
        (
            "Trash",
            rsx! {Icon {data: line_md::Trash, width: "20", height: "20"} },
        ),
    ];

    let drive_list: Vec<(&str, Element)> = vec![
        (
            "Drive 1:",
            rsx! {Icon {data: material_symbols::HardDriveOutline, width: "20", height: "20"} },
        ),
        (
            "Drive 2:",
            rsx! {Icon {data: material_symbols::HardDriveOutline, width: "20", height: "20"} },
        ),
        (
            "Drive 3 :",
            rsx! {Icon {data: material_symbols::HardDriveOutline, width: "20", height: "20"} },
        ),
    ];

    let dropdown_items = rsx! {
        SpItems {
            label: "Proposal".to_string(),
            icon: rsx! {Icon {data: line_md::Folder, width: "20", height: "20", color: FOLDER_COLOR} },
        }
        SpItems {
            label: "Files and...".to_string(),
            icon: rsx! {Icon {data: line_md::Folder, width: "20", height: "20", color: FOLDER_COLOR} },
        }
        SpItems {
            label: "Document.docx".to_string(),
            icon: rsx! {Icon {data: line_md::File, width: "20", height: "20", color: "#4a90d9"} },
        }
        SpItems {
            label: "Text.txt".to_string(),
            icon: rsx! {Icon {data: line_md::File, width: "20", height: "20", color: "#888"} },
        }
    };

    let sp_cloud: Vec<(String, Element, Element)> = vec![
        (
            "iCloud".to_string(),
            rsx! {Icon {data: arcticons::IcloudDrive, width: "20", height: "20", color: GREY} },
            rsx! {Icon {data: fluent::WindowNew20Regular, width: "15", height: "15", color: GREY} },
        ),
        (
            "Google Drive".to_string(),
            rsx! {Icon {data: simple_icons::Googledrive, width: "20", height: "20", color: GREY} },
            rsx! {Icon {data: fluent::WindowNew20Regular, width: "15", height: "15", color: GREY} },
        ),
    ];

    rsx! {
        style { "{side_panel_style()}"},
        div{
            class: "side-panel",
            div{
                class: "app-name-and-icon-container",
                div{
                    class: "app-icon",
                    img {
                        src: APP_ICON,
                    }
                }
                div{
                    class: "app-name",
                    "Leo Explorer"
                }
            }

            for (label, icon) in sp_menu {
                SpMenu {
                    label: label.to_string(),
                    icon,
                    children: dropdown_items.clone(),
                    expand_sp_menu,
                }
            }
            div{
                class: "sp-cloud-wrapper",
                for (label, icon, open_icon) in sp_cloud {
                    SpCloud { label: label.to_string(), icon, open_icon }
                }
            }
            div{
                class: "drive-list-container",
                for (label, icon) in drive_list {
                    SpDrive { label: label.to_string(), icon }
                }
            }
        }
    }
}
