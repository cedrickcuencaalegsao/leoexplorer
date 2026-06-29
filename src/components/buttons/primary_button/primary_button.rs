use crate::core::design::design::primary_button_style;
use dioxus::prelude::*;

#[component]
pub fn PrimaryButton(lable: String) -> Element {
    rsx! {
        button {
            style: primary_button_style(),
            class: "primary-button",
            "{lable}"
        }
    }
}
