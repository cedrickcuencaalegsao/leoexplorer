use crate::shared::enums::{item_type::ItemType, tab_content::TabContent};
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
                content: TabContent::Welcome,
            }]
        }),
        active_tab: use_signal(|| 0),
        selected_item_type: use_signal(|| ItemType::Folder),
        selected_item_path: use_signal(|| String::new()),
    };
    use_context_provider(|| state);
    rsx! {
        {children}
    }
}
