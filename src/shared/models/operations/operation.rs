use dioxus::prelude::*;

pub struct Operation {
    pub icon: Element,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}
