#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::{tab_views::TabViews, title_bar::TitleBar};
use crate::shared::app_state_privoder::AppStateProvider;
use crate::shared::design::design::main_style;

#[component]
pub fn App() -> Element {
    rsx! {
        AppStateProvider {
            children: rsx! {
                style { "{main_style()}" },
                TitleBar {},
                main{
                    TabViews {  }
                }
            }
        }
    }
}
