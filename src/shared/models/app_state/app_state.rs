use crate::shared::enums::{item_type::ItemType, permissions::Permission, tab_content::TabContent};
use crate::shared::models::tab::Tab;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub struct AppState {
    pub expand_sp_menu: Signal<Vec<String>>,
    pub tabs: Signal<Vec<Tab>>,
    pub active_tab: Signal<usize>,
    pub selected_item_type: Signal<ItemType>,
    pub selected_item_path: Signal<String>,
    pub permission: Signal<Permission>,
}

impl AppState {
    pub fn default_tabs() -> Vec<Tab> {
        vec![Tab {
            id: 0,
            title: "Welcome".into(),
            content: TabContent::Welcome,
        }]
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            expand_sp_menu: Signal::new(Vec::new()),
            tabs: Signal::new(AppState::default_tabs()),
            active_tab: Signal::new(0),
            selected_item_type: Signal::new(ItemType::Unknown),
            selected_item_path: Signal::new(String::new()),
            permission: Signal::new(Permission::Guest),
        }
    }
}
