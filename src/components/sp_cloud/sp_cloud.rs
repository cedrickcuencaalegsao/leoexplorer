use crate::shared::design::design::sp_cloud_style;
use dioxus::prelude::*;

#[component]
pub fn SpCloud(
    label: String,
    icon: Element,
    open_icon: Element,
    on_activate: EventHandler<()>,
) -> Element {
    rsx! {
        style { {sp_cloud_style()} },
        div {
            class: "sp-cloud",
            ondoubleclick: move |_| on_activate.call(()),
            div { class: "sp-cloud-children",
                div { class: "sp-cloud-icon", {icon} }
                div { class: "sp-cloud-label", p { "{label}" } }
            }
            div {
                class: "sp-cloud-open",
                onclick: move |event| {
                    event.stop_propagation();
                    on_activate.call(());
                },
                {open_icon}
            }
        }
    }
}
