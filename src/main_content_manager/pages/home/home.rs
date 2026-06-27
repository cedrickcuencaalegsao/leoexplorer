#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Home() -> Element {
    rsx! {
        div{
            h1{"Welcome to Leo Explorer"}
        }
    }
}
