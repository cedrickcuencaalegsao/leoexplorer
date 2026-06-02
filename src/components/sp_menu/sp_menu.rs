#![allow(non_snake_case)]
use crate::shared::{
    design::design::sp_menu_style, models::app_state::AppState, services::open_sp_menu::OpenSpMenu,
    ui::expand_sp_menu::expand_sp_menu::ExpandSpMenu,
};
use dioxus::prelude::*;
use lucide_dioxus::ChevronDown;

#[component]
pub fn SpMenu(label: String, icon: Element, children: Element) -> Element {
    let state: AppState = use_context();
    let expand = state.expand_sp_menu;
    let is_open = expand.read().as_deref() == Some(&label);

    let label_for_open = label.clone();
    let label_for_expand = label.clone();

    rsx! {
        style { "{sp_menu_style()}" }
        div {
            class: "sp-menu-wrapper",
            div {
                class: "sp-menu",
                onclick: move |_| OpenSpMenu(&label_for_open),
                div {
                    class: "sp-menu-children",
                    div { class: "sp-icon", {icon} },
                    div { class: "sp-label", p { "{label}" } }
                },
                if label != "Home" {
                    div {
                        class: if is_open { "sp-chevron sp-chevron-up" } else { "sp-chevron sp-chevron-down" },
                        onclick: move |_| ExpandSpMenu(&label_for_expand, is_open, expand),
                        ChevronDown { size: 12 }
                    }
                }
            }
            if is_open {
                div {
                    class: "sp-menu-dropdown",
                    {children}
                }
            }
        }
    }
}
