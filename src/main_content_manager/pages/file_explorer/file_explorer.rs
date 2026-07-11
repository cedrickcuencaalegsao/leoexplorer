#![allow(non_snake_case)]
use crate::components::{header_bar::HeaderBar, item::Item};
use crate::core::{
    design::file_explorer::file_explorer_style, enums::view_mode::ViewMode,
    services::get_items::GetItems,
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

    let items = GetItems(view_mode);

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
                    for item in items{
                        Item {
                            name: item.name,
                            item_type: item.item_type,
                            date_created: item.date_created,
                            date_modified: item.date_modified,
                            is_dir:item.is_dir,
                            view_mode:view_mode,
                            path: item.path,
                            flag: item.flag,
                        }
                    }
                }
            }
        }
    }
}
