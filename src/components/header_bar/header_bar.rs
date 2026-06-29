#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::header_bar::{
    header_buttons::HeaderButtons, nav_buttons::NavButtons, search_bar::SearchBar,
};
use crate::core::design::header_bar_style::header_bar_style;

#[component]
pub fn HeaderBar() -> Element {
    rsx! {
        style { "{header_bar_style()}" }
        div {
            class: "header-bar",
            div {
                class: "nav-button-container",
                NavButtons{}
            },
            div{
                class: "search-bar-container",
                SearchBar{}
            },
            div {
                class: "header-button-container",
                HeaderButtons{}
            }
        }
    }
}
