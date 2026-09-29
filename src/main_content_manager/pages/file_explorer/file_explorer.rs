#![allow(non_snake_case)]
use crate::components::{header_bar::HeaderBar, item::Item};
use crate::core::{
    design::file_explorer::file_explorer_style,
    enums::view_mode::ViewMode,
    services::get_items::{GetDefaultStartPath, GetItems},
};

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FileExplorerProps {
    pub path: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    List,
    Grid,
    Preview,
}

pub fn FileExplorer(props: FileExplorerProps) -> Element {
    let view_mode = ViewMode::Details;
    let mut path = use_signal(|| String::new());

    use_effect(move || {
        spawn(async move {
            let default_path = GetDefaultStartPath().await;
            path.set(default_path);
        });
    });

    let items = use_resource(move || {
        let path = path.read().clone();
        async move {
            if path.is_empty() {
                return Ok(vec![]); // still resolving, skip fetch
            }
            GetItems(&path, view_mode).await
        }
    });

    rsx! {
        style {"{file_explorer_style()}" },
        div{
            class:"file-explorer-main-container",

            div{
                class:"header-bar-container",
                HeaderBar { }
            }

            div{
                class:"item-container",
                div{
                    class:"items",
                    match &*items.read() {
                        Some(Ok(items)) => rsx! {
                            for item in items {
                                // render each item here, e.g.:
                                div { key: "{item.path}", "{item.name}" }
                            }
                        },
                        Some(Err(e)) => rsx! {
                            div { class: "error-state", "Failed to load: {e}" }
                        },
                        None => rsx! {
                            div { class: "loading-state", "Loading..." }
                        },
                    }
                }
            }
        }
    }
}
