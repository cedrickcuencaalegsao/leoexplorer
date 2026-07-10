#![allow(non_snake_case)]
use crate::icons::*;
use crate::{
    core::{
        constant::constant::FOLDER_COLOR,
        design::item_style::item_style,
        enums::{item_type::ItemType, view_mode::ViewMode},
    },
    icons::line_md,
};
use dioxus::prelude::*;

#[component]
pub fn Item(
    name: String,
    item_type: ItemType,
    is_dir: bool,
    view_mode: ViewMode,
    date_created: String,
    date_modified: String,
    flag: Option<String>,
    path: String,
) -> Element {
    let has_flag = flag.is_some();
    let flag_class = if has_flag {
        "item-flag has-flag"
    } else {
        "item-flag"
    };

    rsx! {
        style { "{item_style()}" }
        div {
            class: "item",
            div {
                class: "item-icon",
                Icon {
                    data: if is_dir { line_md::Folder } else { line_md::File },
                    color: if is_dir { FOLDER_COLOR },
                    width: "24px",
                    height: "24px",
                }
            }
            div {
                class: "item-data",
                div {
                    class: "item-name-container",
                    p {
                        class: "item-name",
                        "{name}"
                    }
                }
                div {
                    class: "{flag_class}",
                    if has_flag {
                        p {
                            class: "item-flag-text",
                            "{flag.as_deref().unwrap_or_default()}"
                        }
                    }
                }
                div {
                    class: "item-metadata-container",
                    p {
                        class: "item-metadata date-created",
                        "Created: {date_created}"
                    }
                    p {
                        class: "item-metadata date-modified",
                        "Modified: {date_modified}"
                    }
                }
            }
            div {
                class: "path",
                p {
                    class: "item-path",
                    "{path}"
                }
            }
        }
    }
}
