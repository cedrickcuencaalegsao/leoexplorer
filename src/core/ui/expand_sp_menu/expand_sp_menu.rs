#![allow(non_snake_case)]
use dioxus::prelude::*;

#[allow(dead_code)]
pub fn ExpandSpMenu(label: &str, mut expand: Signal<Vec<String>>) {
    let mut list = expand.write();
    if let Some(pos) = list.iter().position(|x| x == label) {
        list.remove(pos);
    } else {
        list.push(label.to_string());
    }
}
