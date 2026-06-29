#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn OpenSpMenu(label: &str, mut expand: Signal<Vec<String>>) {
    if label == "Home" {
        expand.set(Vec::new());
    }
}
