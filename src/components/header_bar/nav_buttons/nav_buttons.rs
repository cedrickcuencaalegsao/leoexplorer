#![allow(non_snake_case)]
use crate::icons::*;
use dioxus::prelude::*;

#[component]
pub fn NavButtons() -> Element {
    rsx! {
        div{
            class: "nav-buttons",
            div{
                class: "nav-button",
                Icon{data: solar::ArrowLeftBroken}
            }
            div{
                class: "nav-button",
                Icon{data: solar::ArrowRightBroken}
            }
        }
    }
}
