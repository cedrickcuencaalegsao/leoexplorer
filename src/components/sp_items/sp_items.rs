use crate::shared::design::design::sp_items_style;
use dioxus::prelude::*;

#[component]
pub fn SpItems(label: String, icon: Element) -> Element {
    rsx! {
        style { "{sp_items_style()}" }
        div { class: "sp-items",
            div { class: "sp-items-icon", {icon} },
            div { class: "sp-items-label", p { "{label}" } }
        }
    }
}
