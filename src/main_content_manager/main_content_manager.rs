use crate::components::{header_bar::HeaderBar, side_panel::SidePanel};
use crate::main_content_manager::pages::{
    account::Account, cloud_web_view::CloudWebView, dashboard::Dashboard, editor::Editor,
    file_explorer::FileExplorer, help::Help, home::Home, settings::Settings, terminal::Terminal,
    welcome::Welcome,
};
use crate::core::{enums::tab_content::TabContent, models::tab::Tab};
use dioxus::prelude::*;

macro_rules! with_layout {
    ($content:expr) => {
        rsx! {
            div {
                class: "app-container",
                div { class: "header-bar-container", HeaderBar {} }
                div { class: "body-container",
                    div { class: "side-panel-container", SidePanel {} }
                    div { class: "main-panel-container", {$content} }
                    div { class: "preview-panel-container" }
                }
            }
        }
    };
}

#[component]
pub fn MainContentManager(tab: Tab, is_active: bool) -> Element {
    match &tab.content {
        TabContent::GDrive => rsx! {
            CloudWebView { url: "https://drive.google.com", label: "gdrive", is_active }
        },
        TabContent::ICloud => rsx! {
            CloudWebView { url: "https://www.icloud.com", label: "icloud", is_active }
        },
        TabContent::GMail => rsx! {
            CloudWebView { url: "https://mail.google.com", label: "gmail", is_active }
        },
        TabContent::Welcome => rsx! { Welcome {} },
        TabContent::Dashboard => with_layout!(rsx! { Dashboard {} }),
        TabContent::Account => with_layout!(rsx! { Account {} }),
        TabContent::Settings => with_layout!(rsx! { Settings {} }),
        TabContent::Terminal => with_layout!(rsx! { Terminal {} }),
        TabContent::Help => with_layout!(rsx! { Help {} }),
        TabContent::Home => with_layout!(rsx! { Home {} }),
        TabContent::FileExplorer(path) => {
            with_layout!(rsx! { FileExplorer { path: path.clone() } })
        }
        TabContent::Editor(path) => with_layout!(rsx! { Editor { path: path.clone() } }),
    }
}
