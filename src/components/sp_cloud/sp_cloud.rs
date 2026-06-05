use dioxus::prelude::*;

#[component]
pub fn SpCloud(label: String, icon: Element) -> Element {
    rsx! {
        div {
            class: "sp-cloud",
            div { class: "sp-cloud-icon", {icon} }
            div { class: "sp-cloud-label", p{"{label}"} }
        }
    }
}
