#![allow(non_snake_case)]
use crate::shared::design::design::sp_drive_style;
use dioxus::prelude::*;

#[component]
pub fn SpDrive(label: String, icon: Element) -> Element {
    rsx! {
        style {
            {sp_drive_style()}
        }
        div{
            class: "sp-drive",
            div{
                class: "sp-drive-children",
                div{
                    class: "sp-drive-icon",
                    {icon}
                }
                div{
                    class: "sp-drive-label",
                    {label}
                }
            }
        }
    }
}
