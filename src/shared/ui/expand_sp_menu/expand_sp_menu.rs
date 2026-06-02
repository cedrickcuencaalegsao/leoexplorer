#![allow(non_snake_case)]
use dioxus::prelude::*;

#[allow(dead_code)]
pub fn ExpandSpMenu(label: &str, is_open: bool, mut expand: Signal<Option<String>>) {
    if label == "Home" {
        expand.set(None);
        return;
    }
    if is_open {
        expand.set(None);
    } else {
        expand.set(Some(label.to_string()))
    }
}
