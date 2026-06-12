use crate::components::tab_content_view::TabContentView;
use crate::shared::models::app_state::AppState;
use dioxus::prelude::*;

#[component]
pub fn TabViews() -> Element {
    let state = use_context::<AppState>();
    let tabs = state.tabs.clone();
    let active = state.active_tab.clone();
    rsx! {
        for tab in tabs.read().iter() {
            div{
                key: "{tab.id}",
                class: if *active.read() == tab.id { "tab-view active" } else { "tab-view" },
                TabContentView { tab: tab.clone() }
            }
        }
    }
}
