use crate::shared::models::tab::Tab;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub struct AppState {
    pub expand_sp_menu: Signal<Vec<String>>,
    pub tabs: Signal<Vec<Tab>>,
    pub active_tab: Signal<usize>,
}
