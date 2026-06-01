#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::side_panel::SidePanel;
use crate::shared::app_state_privoder::AppStateProvider;
use crate::shared::design::design::main_style;

pub fn App() -> Element {
    rsx! {
        AppStateProvider {
            children: rsx! {
                style { "{main_style()}" },
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
    }
}
