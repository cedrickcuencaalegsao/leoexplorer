#![allow(non_snake_case)]
use crate::core::{
    constant::constant::{GDRIVE_ICONS, GMAIL_ICONS, ICLOUD_ICONS},
    design::welcome_page::welcome_page_style,
    enums::tab_content::TabContent,
    models::app_state::AppState,
};
use dioxus::prelude::*;

#[component]
pub fn Welcome() -> Element {
    let state = use_context::<AppState>();
    let mut router = state.router;

    let mut nav = move |content: TabContent| router.write().navigate(content);

    rsx! {
        style { "{welcome_page_style()}" }
        div{
            class:"welcome-page",
            h1 { "Welcome" }
            p { class: "welcome-subtitle", "Open a page in a new tab to get started."}

            p { class: "welcome-section-lable", "Page"}
            div{
                class: "welcome-grid",
                WelcomeCard { label: "Dashboard", onclick: move |_| nav(TabContent::Dashboard), }
                WelcomeCard { label: "Home", onclick: move |_| nav(TabContent::Home), }
                WelcomeCard { label: "File Explorer", onclick: move |_| nav(TabContent::FileExplorer("".to_string())), }
                WelcomeCard { label: "Terminal", onclick: move |_| nav(TabContent::Terminal), }
            }

            p { class: "welcome-section-lable", "Cloud"}
            div{
                class: "welcome-grid",
                WelcomeCard { label: "GMail", onclick: move |_| nav(TabContent::GMail), }
                WelcomeCard { label: "GDrive", onclick: move |_| nav(TabContent::GDrive), }
                WelcomeCard { label: "ICloud", onclick: move |_| nav(TabContent::ICloud), }
            }

            p { class: "welcome-section-lable", "System"}
            div{
                class: "welcome-grid",
                WelcomeCard { label: "Account", onclick: move |_| nav(TabContent::Account), }
                WelcomeCard { label: "Settings", onclick: move |_| nav(TabContent::Settings), }
                WelcomeCard { label: "Help", onclick: move |_| nav(TabContent::Help), }
            }
        }
    }
}

#[component]
fn WelcomeCard(label: &'static str, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div{
            class: "welcome-card",
            onclick: move |event| onclick.call(event),
            div{
                class: "welcome-card-icon",
                
            }
            span{
                class: "welcome-card-label",
                "{label}"
            }
        }
    }
}
