use crate::shared::design::design::secondary_button_style;
use dioxus::prelude::*;

#[component]
pub fn SecondaryButton(lable: String) -> Element {
    rsx! {
        button {
            style: secondary_button_style(),
            class: "secondary-button",
            {lable}
        }
    }
}
