use crate::components::sp_menu::SpMenu;
use crate::shared::design::design::side_panel_style;
use dioxus::prelude::*;

#[component]
pub fn SidePanel() -> Element {
    let sp_menu = vec![
        "Home",
        "Desktop",
        "Downloads",
        "Photos",
        "Music",
        "Movies ",
        "Trash",
    ];
    let drive_list = vec!["Drive 1", "Drive 2", "Drive 3"];
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
                for item in sp_menu {
                    SpMenu { label: item.to_string() }
                }
            }
            div{
                class: "drive-list-container",
                for item in drive_list{
                    p { "{item}" }
                }
            }
        }
    }
}
