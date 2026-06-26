use crate::components::folder_options::FolderOptions;
use crate::shared::design::design::sp_items_style;
use crate::shared::enums::item_type::ItemType;
use dioxus::prelude::*;

#[component]
pub fn SpItems(label: String, icon: Element, item_type: ItemType) -> Element {
    let mut show_folder_options = use_signal(|| false);
    let mut menu_x = use_signal(|| 0.0f64);
    let mut menu_y = use_signal(|| 0.0f64);

    rsx! {
        style { "{sp_items_style()}" }

        if *show_folder_options.read(){
            div{
                class:"ctx-overlay",
                onclick: move |_| show_folder_options.set(false),
                oncontextmenu: move |event|{
                    event.prevent_default();
                    show_folder_options.set(false);
                }
            }
            FolderOptions { x: *menu_x.read(), y: *menu_y.read(), item_type, on_close: move |_| show_folder_options.set(false) }
        }

        div { class: "sp-items",
            oncontextmenu: move |event|{
                event.prevent_default();
                let coords = event.client_coordinates();
                menu_x.set(coords.x);
                menu_y.set(coords.y);
                show_folder_options.set(true);
            },
            div { class: "sp-items-icon", {icon} },
            div { class: "sp-items-label", p { "{label}" } }
        }
    }
}
