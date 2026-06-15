#![allow(non_snake_case)]
use crate::components::{sp_cloud::SpCloud, sp_drive::SpDrive, sp_items::SpItems, sp_menu::SpMenu};
use crate::icons::*;
use crate::shared::{
    constant::constant::{FOLDER_COLOR, GREY},
    design::design::side_panel_style,
    enums::tab_content::TabContent,
    models::{app_state::AppState, tab::Tab},
};
use dioxus::prelude::*;
const ICLOUD_ICONS: Asset = asset!("/assets/icons/apple-icons/iCloud.png");
const GDRIVE_ICONS: Asset = asset!("/assets/icons/google-icons/googledrive.png");
const GMAIL_ICONS: Asset = asset!("/assets/icons/google-icons/gmail.png");

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

    let state = use_context::<AppState>();
    let mut tabs = state.tabs;
    let mut active_tabs = state.active_tab;

    let drive_list: Vec<(&str, Element, f64, f64)> = vec![
        (
            "Drive 1:",
            rsx! {Icon {data: material_symbols::HardDriveOutline, width: "20", height: "20"} },
            50.0,
            100.0,
        ),
        (
            "Drive 2:",
            rsx! {Icon {data: material_symbols::HardDriveOutline, width: "20", height: "20"} },
            75.0,
            100.0,
        ),
        (
            "Drive 3 :",
            rsx! {Icon {data: material_symbols::HardDriveOutline, width: "20", height: "20"} },
            25.0,
            100.0,
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

    let sp_cloud: Vec<(String, Element, Element, TabContent)> = vec![
        (
            "iCloud".to_string(),
            rsx! {img {src: ICLOUD_ICONS, width: "20", height: "20"}},
            rsx! {Icon {data: fluent::WindowNew20Regular, width: "15", height: "15", color: GREY} },
            TabContent::ICloud,
        ),
        (
            "Google Drive".to_string(),
            rsx! {img {src: GDRIVE_ICONS, width: "20", height: "20"}},
            rsx! {Icon {data: fluent::WindowNew20Regular, width: "15", height: "15", color: GREY} },
            TabContent::GDrive,
        ),
        (
            "Gmail".to_string(),
            rsx! {img {src: GMAIL_ICONS, width: "20", height: "20"}},
            rsx! {Icon {data: fluent::WindowNew20Regular, width: "15", height: "15", color: GREY} },
            TabContent::GMail,
        ),
    ];

    let sp_cloud_data = sp_cloud
        .into_iter()
        .map(|(label, icon, open_icon, content)| (label.clone(), label, icon, open_icon, content))
        .collect::<Vec<_>>();

    rsx! {
        style { "{side_panel_style()}"},
        div{
            class: "side-panel",
            div{
                class: "library",
                div{
                    class: "sp-header",
                    p { "Library" }
                }
                div{
                    class: "library-wrapper",
                    for (label, icon) in sp_menu {
                        SpMenu {
                            label: label.to_string(),
                            icon,
                            children: dropdown_items.clone(),
                            expand_sp_menu,
                        }
                    }
                }
            }

            div{
                class: "remote",
                p { class: "sp-header", "Remote" }
                div{
                    class: "sp-cloud-wrapper",
                    for (label_tab, label, icon, open_icon, content) in sp_cloud_data {
                        SpCloud {
                            label,
                            icon,
                            open_icon,
                            on_activate: move |_| {
                                let new_id = tabs.read().len();
                                tabs.write().push(Tab {
                                    id: new_id,
                                    title: label_tab.clone(),
                                    content: content.clone(),
                                });
                                active_tabs.set(new_id);
                            },
                        }
                    }
                }
            }

            div{
                class: "drive",
                p { class: "sp-header", "Drive" }
                div{
                    class: "drive-list-container",
                    for (label, icon, used_gb, total_gb) in drive_list {
                        SpDrive { label: label.to_string(), icon, used_gb, total_gb }
                    }
                }
            }
        }
    }
}
