use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
pub struct AppState {
    pub expand_sp_menu: Signal<Option<String>>,
}
