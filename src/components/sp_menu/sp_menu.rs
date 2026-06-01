#![allow(non_snake_case)]
use crate::shared::design::design::sp_menu_style;
use dioxus::prelude::*;

#[component]
pub fn SpMenu(label: String, icon: Element) -> Element {
    rsx! {
        style {
            "{sp_menu_style()}"
        }
        div{
            class: "sp-menu" } else { "sp-menu sp-menu-collapsed" ,
            div{
                class: "sp-menu-children",
                div{
                    class: "sp-icon",
                    {icon},
                },
                div {
                    class: "sp-label",
                    p{
                        "{label}"
                    }
                }
            }
        }
    }
}
