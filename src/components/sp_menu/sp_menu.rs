use crate::shared::design::design::sp_menu_style;
use dioxus::prelude::*;

#[component]
pub fn SpMenu(label: String) -> Element {
    rsx! {
        style {
            "{sp_menu_style()}"
        }
        div{
            class: "sp-menu",
            div{
                class: "sp-menu-children",
                p{
                    "{label}"
                }
            }
        }
    }
}
