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
            {
                let is_active = *active.read() == tab.id;
                rsx! {
                    div {
                        key: "{tab.id}",
                        // CSS hides the anchor div but the native webview
                        // is hidden via hide_cloud_view command instead
                        class: if is_active { "tab-view active" } else { "tab-view" },
                        TabContentView { tab: tab.clone(), is_active }
                    }
                }
            }
        }
    }
}
