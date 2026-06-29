use crate::core::enums::tab_content::TabContent;
use crate::core::enums::{item_type::ItemType, permissions::Permission};
use crate::core::models::tab::Tab;
use crate::core::{models::app_state::AppState, services::page_router::PageRouter};
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
        router: use_signal(|| PageRouter::default()),
    };

    let mut tabs = state.tabs;
    let mut active = state.active_tab;
    let mut router = state.router;

    use_effect(move || {
        let page = router.read().page.clone();

        if matches!(page, TabContent::Welcome) {
            return;
        }

        let new_id = tabs
            .read()
            .iter()
            .map(|tab| tab.id)
            .max()
            .map(|id| id + 1)
            .unwrap_or(0);

        tabs.write().push(Tab {
            id: new_id,
            title: page.title().into(),
            content: page,
        });

        active.set(new_id);

        router.write().navigate(TabContent::Welcome);
    });

    use_context_provider(|| state);

    rsx! {
        {children}
    }
}
