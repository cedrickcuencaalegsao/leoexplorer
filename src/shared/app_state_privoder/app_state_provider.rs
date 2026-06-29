use crate::shared::enums::{item_type::ItemType, permissions::Permission};
use crate::shared::models::app_state::AppState;
use dioxus::prelude::*;

#[component]
pub fn AppStateProvider(children: Element) -> Element {
    let state = AppState {
        expand_sp_menu: use_signal(|| Vec::<String>::new()),
        tabs: use_signal(|| AppState::default_tabs()),
        active_tab: use_signal(|| 0),
        selected_item_type: use_signal(|| ItemType::Folder),
        selected_item_path: use_signal(|| String::new()),
        permission: use_signal(|| Permission::Admin),
    };
    use_context_provider(|| state);

    rsx! {
        {children}
    }
}
