use dioxus::prelude::*;

pub struct FolderOperation {
    pub icon: Element,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}
