#![allow(non_snake_case)]
use crate::icons::*;
use dioxus::prelude::*;

use crate::icons::material_symbols;

#[component]
pub fn HeaderButtons() -> Element {
    rsx! {
        div{
            class: "header-buttons",
            div{
                class:"button",
                Icon{ data: material_symbols::SettingsOutlineRounded},
            }
            div{
                class:"button",
                Icon{ data: material_symbols::PersonOutlineRounded},
            }
            div{
                class:"button",
                Icon{ data: simple_icons::Googlegemini},
            }
        }
    }
}
