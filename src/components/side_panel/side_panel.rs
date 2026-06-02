#![allow(non_snake_case)]
use crate::components::sp_drive::SpDrive;
use crate::components::sp_items::SpItems;
use crate::components::sp_menu::SpMenu;
use crate::shared::{constant::constant::FOLDER_COLOR, design::design::side_panel_style};
use dioxus::prelude::*;
use lucide_dioxus::{
    File, Files, Folder, FolderDown, HardDrive, House, Images, Monitor, Music, Trash2, Video,
};

#[component]
pub fn SidePanel() -> Element {
    let sp_menu: Vec<(&str, Element)> = vec![
        ("Home", rsx! {House {size: 20} }),
        ("Desktop", rsx! {Monitor {size: 20} }),
        ("Documents", rsx! {Files {size: 20} }),
        ("Downloads", rsx! {FolderDown {size: 20} }),
        ("Photos", rsx! {Images {size: 20}}),
        ("Music", rsx! {Music {size: 20}}),
        ("Movies", rsx! {Video {size: 20}}),
        ("Trash", rsx! {Trash2 {size: 20}}),
    ];

    let drive_list: Vec<(&str, Element)> = vec![
        ("Drive 1:", rsx! {HardDrive {size: 20} }),
        ("Drive 2:", rsx! {HardDrive {size: 20} }),
        ("Drive 3 :", rsx! {HardDrive {size: 20} }),
    ];

    let dropdown_items = rsx! {
        SpItems {
            label: "Proposal".to_string(),
            icon: rsx! { Folder { size: 18, color: FOLDER_COLOR} },
        }
        SpItems {
            label: "Files and...".to_string(),
            icon: rsx! { Folder { size: 18, color: FOLDER_COLOR } },
        }
        SpItems {
            label: "Document.docx".to_string(),
            icon: rsx! { File { size: 18, color: "#4a90d9" } },
        }
        SpItems {
            label: "Text.txt".to_string(),
            icon: rsx! { File { size: 18, color: "#888" } },
        }
    };

    rsx! {
        style { "{side_panel_style()}"},
        div{
            class: "side-panel",
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
