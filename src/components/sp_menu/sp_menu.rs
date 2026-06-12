#![allow(non_snake_case)]
use crate::shared::{
    design::design::sp_menu_style, services::open_sp_menu::OpenSpMenu,
    ui::expand_sp_menu::expand_sp_menu::ExpandSpMenu,
};
use dioxus::prelude::*;
use lucide_dioxus::ChevronDown;

#[component]
pub fn SpMenu(
    label: String,
    icon: Element,
    children: Element,
    expand_sp_menu: Signal<Vec<String>>,
) -> Element {
    let expand = expand_sp_menu;
    let is_open = expand.read().contains(&label);
    let label_for_open = label.clone();
    let label_for_expand = label.clone();
    rsx! {
        style { "{sp_menu_style()}" }
        div {
            class: "sp-menu-wrapper",
            div {
                class: if is_open { "sp-menu sp-menu-open" } else { "sp-menu" },
                onclick: move |_| OpenSpMenu(&label_for_open, expand),
                div {
                    class: "sp-menu-children",
                    div { class: "sp-icon", {icon} },
                    div { class: "sp-label", p { "{label}" } }
                },
                if label != "Home" {
                    div {
                        class: if is_open { "sp-chevron sp-chevron-up" } else { "sp-chevron sp-chevron-down" },
                        onclick: move |_| ExpandSpMenu(&label_for_expand, expand),
                        ChevronDown { size: 12 }
                    }
                }
            }
            if is_open {
                div {
                    class: if is_open { "sp-menu-dropdown open" } else { "sp-menu-dropdown" },
                    {children}
                }
            }
        }
    }
}
