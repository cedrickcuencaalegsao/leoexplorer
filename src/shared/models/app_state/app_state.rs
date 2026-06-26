use crate::shared::enums::item_type::ItemType;
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
}
