use crate::core::constant::{
    file_operations::FileOperations, folder_operations::folder_operations,
};
use crate::core::design::options_style::options_style;
use crate::core::enums::{file_type::FileType, item_type::ItemType};
use dioxus::prelude::*;

#[component]
pub fn Options(x: f64, y: f64, item_type: ItemType, on_close: EventHandler<()>) -> Element {
    let sections = match item_type {
        ItemType::Folder => folder_operations(),

        ItemType::File(file_type) => match file_type {
            FileType::Document(_) => FileOperations(),
            FileType::Image(_) => FileOperations(),
            FileType::Video(_) => FileOperations(),
            FileType::Developer(_) => FileOperations(),
            FileType::Unknown => FileOperations(),
            _ => FileOperations(),
        },

        ItemType::Unknown | ItemType::Default => vec![],
    };
    let total = sections.len();

    rsx! {
        style { "{options_style()}" }
        div {
            class: "folder-ctx-menu",
            style: "
                left: {x}px;
                top: {y}px;
                max-height: calc(100vh - {y}px - 16px);
                overflow-y: scroll;
                overflow-x: hidden;
            ",
            onclick: move |e| e.stop_propagation(),
            oncontextmenu: move |e| e.prevent_default(),
            for (i, section) in sections.into_iter().enumerate() {
                div {
                    class: "ctx-section {section.section_class}",
                    for item in section.items {
                        div {
                            class: "ctx-item {section.item_class}",
                            onclick: {
                                let on_close = on_close.clone();
                                move |_| on_close.call(())
                            },
                            span { class: "ctx-item-icon", {item.icon} }
                            span { class: "ctx-item-label", "{item.label}" }
                            if let Some(sc) = item.shortcut {
                                span { class: "ctx-item-shortcut", "{sc}" }
                            }
                        }
                    }
                }
                if i < total - 1 {
                    hr { class: "ctx-divider" }
                }
            }
        }
    }
}
