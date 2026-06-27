#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Help() -> Element {
    rsx! {
        div{
            h1 { "Help" }
        }
    }
}
