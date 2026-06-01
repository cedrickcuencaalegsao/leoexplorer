use crate::shared::models::app_state::AppState;
use dioxus::prelude::*;

#[component]
pub fn AppStateProvider(children: Element) -> Element {
    let state = AppState {
        expand_sp_menu: use_signal(|| None::<String>),
    };
    use_context_provider(|| state);
    rsx! {
        {children}
    }
}
