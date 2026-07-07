#![allow(non_snake_case)]
use crate::icons::*;
use dioxus::prelude::*;

#[component]
pub fn SearchBar() -> Element {
    rsx! {
        div{
            class: "search-bar",
            div{
                class: "search-bar-icon-wrapper",
                div{
                    class: "search-bar-icon",
                    Icon{ data: material_symbols::SearchRounded}
                }
            },
            div{
                class: "search-bar-input-wrapper",
                input {
                    type: "text",
                    placeholder: "Search...",
                }
            }
        }
    }
}
