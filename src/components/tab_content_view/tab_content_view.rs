use crate::main_content_manager::main_content_manager::MainContentManager;
use crate::core::constant::allowed_tabs::is_tab_allowed;
use crate::core::design::access_denied_style::access_denied_style;
use crate::core::models::{app_state::AppState, tab::Tab};
use dioxus::prelude::*;

#[component]
pub fn TabContentView(tab: Tab, is_active: bool) -> Element {
    let state = use_context::<AppState>();
    let permission = state.permission.read().clone();

    if !is_tab_allowed(&permission, &tab.content) {
        return rsx! {
            style { "{access_denied_style()}" }
            div {
                class: "access-denied",
                div {
                    class: "access-denied-box",
                    span { class: "access-denied-icon", "" }
                    h2 { "Access Denied" }
                    p { "You don't have permission to view this page." }
                    p {
                        class: "access-denied-role",
                        "Current role: {permission:?}"
                    }
                }
            }
        };
    }

    rsx! {
        MainContentManager { tab, is_active }
    }
}
