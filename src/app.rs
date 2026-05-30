#![allow(non_snake_case)]

use crate::components::side_panel::SidePanel;

use dioxus::prelude::*;

// use serde::{Deserialize, Serialize};
// use wasm_bindgen::prelude::*;

pub fn App() -> Element {
    rsx! {
        main {
            div{
                class: "app-container",
                div{
                    class: "side-panel-container",
                    SidePanel {}
                },
                div{
                    class: "main-panel-container",
                },
                div{
                    class: "preview-panel-container",
                },
                div{
                    class: "dynamic-sidebar-container",
                },
            }
        }
    }
}
