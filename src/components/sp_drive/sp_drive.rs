#![allow(non_snake_case)]
use crate::core::design::design::sp_drive_style;
use dioxus::prelude::*;

#[component]
pub fn SpDrive(label: String, icon: Element, used_gb: f64, total_gb: f64) -> Element {
    let used_percent = if total_gb > 0.0 {
        (used_gb / total_gb * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let free_gb = (total_gb - used_gb).max(0.0);

    rsx! {
        style { {sp_drive_style()} }
        div {
            class: "sp-drive",
            div {
                class: "sp-drive-icon",
                {icon}
            }
            div {
                class: "sp-drive-info",
                div { class: "sp-drive-label", "{label}" }
                div {
                    class: "sp-drive-bar",
                    div {
                        class: "sp-drive-bar-used",
                        style: "width: {used_percent}%;",
                    }
                }
                div {
                    class: "sp-drive-stats",
                    div {
                        class: "sp-drive-stat",
                        span { class: "sp-drive-dot sp-drive-dot-free" }
                        "free {free_gb:.0} out of {total_gb:.0} GB"
                    }
                    div {
                        class: "sp-drive-stat",
                        span { class: "sp-drive-dot sp-drive-dot-used" }
                        "used {used_gb:.0} GB out of {total_gb:.0} GB"
                    }
                }
            }
        }
    }
}
