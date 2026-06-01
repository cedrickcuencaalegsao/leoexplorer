#![allow(non_snake_case)]
use crate::components::sp_drive::SpDrive;
use crate::components::sp_menu::SpMenu;
use crate::shared::design::design::side_panel_style;
use dioxus::prelude::*;
use lucide_dioxus::{FolderDown, HardDrive, House, Images, Monitor, Music, Trash, Video};

#[component]
pub fn SidePanel() -> Element {
    let sp_menu: Vec<(&str, Element)> = vec![
        ("Home", rsx! {House {size: 16} }),
        ("Desktop", rsx! {Monitor {size: 16} }),
        ("Downloads", rsx! {FolderDown {size: 16} }),
        ("Photos", rsx! {Images {size: 16}}),
        ("Music", rsx! {Music {size: 16}}),
        ("Movies", rsx! {Video {size: 16}}),
        ("Trash", rsx! {Trash {size: 16}}),
    ];

    let drive_list: Vec<(&str, Element)> = vec![
        ("Drive 1:", rsx! {HardDrive {size: 16} }),
        ("Drive 2:", rsx! {HardDrive {size: 16} }),
        ("Drive 3 :", rsx! {HardDrive {size: 16} }),
    ];

    rsx! {
        style { "{side_panel_style()}"},
        div{
            class: "side-panel",
            div{
                class:"app-name-container",
                h1 { "Leo Explorer" }
            }
            div{
                class: "menu-container",
                for (label, icon) in sp_menu {
                    SpMenu { label: label.to_string(), icon }
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
