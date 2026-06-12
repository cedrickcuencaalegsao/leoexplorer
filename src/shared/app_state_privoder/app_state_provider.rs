use crate::shared::models::{app_state::AppState, tab::Tab};
use dioxus::prelude::*;

#[component]
pub fn AppStateProvider(children: Element) -> Element {
    let state = AppState {
        expand_sp_menu: use_signal(|| Vec::<String>::new()),
        tabs: use_signal(|| {
            vec![Tab {
                id: 0,
                title: "New Tab".into(),
                path: "".into(),
            }]
        }),
        active_tab: use_signal(|| 0),
    };
    use_context_provider(|| state);
    rsx! {
        {children}
    }
}
