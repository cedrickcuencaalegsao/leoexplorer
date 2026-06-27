use crate::components::side_panel::SidePanel;
use crate::main_content_manager::pages::{cloud_web_view::CloudWebView, welcome::Welcome};
use crate::shared::{enums::tab_content::TabContent, models::tab::Tab};
use dioxus::prelude::*;

#[component]
pub fn TabContentView(tab: Tab, is_active: bool) -> Element {
    match &tab.content {
        TabContent::GDrive => rsx! {
            CloudWebView {
                url: "https://drive.google.com",
                label: "gdrive",
                is_active,
            }
        },
        TabContent::ICloud => rsx! {
            CloudWebView {
                url: "https://www.icloud.com",
                label: "icloud",
                is_active,
            }
        },
        TabContent::GMail => rsx! {
            CloudWebView {
                url: "https://mail.google.com",
                label: "gmail",
                is_active,
            }
        },
        _ => rsx! {
            div {
                class: "app-container",
                div { class: "side-panel-container", SidePanel {} }
                div { class: "main-panel-container", Welcome {} }
                div { class: "preview-panel-container" }
            }
        },
    }
}
