#![allow(non_snake_case)]
use crate::shared::{design::design::sp_menu_style, models::app_state::AppState};
use dioxus::prelude::*;
use lucide_dioxus::ChevronDown;

#[component]
pub fn SpMenu(label: String, icon: Element, children: Element) -> Element {
    let state: AppState = use_context();
    let mut expand = state.expand_sp_menu;
    let is_open = expand.read().as_deref() == Some(&label);
    let mut click_timer = use_signal(|| None::<dioxus_core::Task>);

    let label_for_rsx = label.clone();
    let label_for_open = label.clone();

    let handle_click = move |_| {
        if let Some(handle) = click_timer.take() {
            handle.cancel();
            println!("{} (double-click)", label_for_open);
            return;
        }
        let label_clone = label_for_open.clone();
        let is_open_snapshot = is_open;
        let handle = spawn(async move {
            gloo_timers::future::TimeoutFuture::new(100).await;
            if is_open_snapshot {
                expand.set(None);
            } else {
                expand.set(Some(label_clone));
            }
        });
        click_timer.set(Some(handle));
    };

    rsx! {
        style { "{sp_menu_style()}" }
        div {
            class: "sp-menu-wrapper",
            div {
                class: "sp-menu",
                onclick: handle_click,
                div {
                    class: "sp-menu-children",
                    div { class: "sp-icon", {icon} },
                    div { class: "sp-label", p { "{label_for_rsx}" } }
                },
                if label_for_rsx != "Home" {
                    div {
                        class: if is_open { "sp-chevron sp-chevron-up" } else { "sp-chevron sp-chevron-down" },
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
