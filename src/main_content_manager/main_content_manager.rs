use crate::shared::{enums::tab_content::TabContent, models::tab::Tab};
use dioxus::prelude::*;

#[component]
pub fn MainContentManager(tab: Tab) -> Element {
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
