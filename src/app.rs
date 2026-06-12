#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::side_panel::SidePanel;
use crate::components::title_bar::TitleBar;
use crate::shared::app_state_privoder::AppStateProvider;
use crate::shared::design::design::main_style;
use crate::shared::enums::tab_content::TabContent;
use crate::shared::{models::app_state::AppState, models::tab::Tab};

pub fn App() -> Element {
    rsx! {
        AppStateProvider {
            children: rsx! {
                style { "{main_style()}" },
                TitleBar {},
                main{
                    TabViews {  }
                }
            }
        }
    }
}

#[component]
fn TabViews() -> Element {
    let state = use_context::<AppState>();
    let tabs = state.tabs.clone();
    let active = state.active_tab.clone();
    rsx! {
        for tab in tabs.read().iter() {
            div{
                key: "{tab.id}",
                class: if *active.read() == tab.id { "tab-view active" } else { "tab-view" },
                TabContentView { tab: tab.clone() }
            }
        }
    }
}

#[component]
fn TabContentView(tab: Tab) -> Element {
    let is_file_explorer = matches!(tab.content, TabContent::FileExplorer(_));

    rsx! {
        div{
            class: "app-container",
            div{
                class: "side-panel-container",
                SidePanel {}
            },
            div{
                class: "main-panel-container",
                MainContent { tab: tab.clone() }
            },
            div{
                class: "preview-panel-container",
            },
            div{
                class: "dynamic-sidebar-container",
            },
        }
    }
}

#[component]
fn MainContent(tab: Tab) -> Element {
    rsx! {
        match &tab.content {
            TabContent::Welcome => rsx! { div { "Welcome screen" } },
            TabContent::FileExplorer(path) => rsx! { div { "Browsing: {path}" } },
            TabContent::Editor(path) => rsx! { div { "Editing: {path}" } },
            TabContent::Settings => rsx! { div { "Settings" } },
            TabContent::Terminal => rsx! { div { "Terminal" } },
            TabContent::Help => rsx! { div { "Help" } },
            TabContent::Account => rsx! { div { "Account" } },
            TabContent::GMail => rsx! { div { "GMail" } },
            TabContent::GDrive => rsx! { div { "GDrive" } },
            TabContent::Dashboard => rsx! { div { "Dashboard" } },
        }
    }
}
