use crate::components::side_panel::SidePanel;
use crate::main_content_manager::MainContentManager;
use crate::shared::{enums::tab_content::TabContent, models::tab::Tab};
use dioxus::prelude::*;

#[component]
pub fn TabContentView(tab: Tab) -> Element {
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
                MainContentManager { tab: tab.clone() }
            },
            div{
                class: "preview-panel-container",
            },
            if is_file_explorer {
                div{
                    class: "dynamic-sidebar-container",
                }
            }
        }
    }
}
