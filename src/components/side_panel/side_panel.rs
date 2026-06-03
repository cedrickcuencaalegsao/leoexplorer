#![allow(non_snake_case)]
use crate::components::sp_drive::SpDrive;
use crate::components::sp_items::SpItems;
use crate::components::sp_menu::SpMenu;
use crate::icons::*;
use crate::shared::{constant::constant::FOLDER_COLOR, design::design::side_panel_style};
use dioxus::prelude::*;
use lucide_dioxus::{File, Folder, HardDrive};

pub const APP_ICON: Asset = asset!("/assets/icons/app-icon.png");

#[component]
pub fn SidePanel() -> Element {
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
                    children: dropdown_items.clone()
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
