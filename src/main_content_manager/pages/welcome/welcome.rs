#![allow(non_snake_case)]
use crate::core::{
    constant::constant::{GDRIVE_ICONS, GMAIL_ICONS, ICLOUD_ICONS},
    design::welcome_page::welcome_page_style,
    enums::tab_content::TabContent,
    models::app_state::AppState,
};
use crate::icons::*;
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
                WelcomeCard { label: "Dashboard", icon: rsx!{Icon{data: material_symbols::DashboardOutlineRounded, width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::Dashboard), }
                WelcomeCard { label: "Home", icon: rsx!{Icon{data: material_symbols::HomeOutlineRounded, width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::Home), }
                WelcomeCard { label: "File Explorer", icon: rsx!{Icon{data: material_symbols::FolderOutlineRounded, width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::FileExplorer("".to_string())), }
                WelcomeCard { label: "Terminal", icon: rsx!{Icon{data: material_symbols::TerminalRounded, width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::Terminal), }
            }

            p { class: "welcome-section-lable", "Cloud"}
            div{
                class: "welcome-grid",
                WelcomeCard { label: "GMail", icon: rsx!{img { src: GMAIL_ICONS , width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::GMail), }
                WelcomeCard { label: "GDrive", icon: rsx!{img { src: GDRIVE_ICONS , width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::GDrive), }
                WelcomeCard { label: "ICloud", icon: rsx!{img { src: ICLOUD_ICONS , width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::ICloud), }
            }

            p { class: "welcome-section-lable", "System"}
            div{
                class: "welcome-grid",
                WelcomeCard { label: "Account", icon: rsx!{Icon{data: material_symbols::PersonOutlineRounded, width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::Account), }
                WelcomeCard { label: "Settings", icon: rsx!{Icon{data: material_symbols::SettingsOutlineRounded, width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::Settings), }
                WelcomeCard { label: "Help", icon: rsx!{Icon{data: material_symbols::HelpOutlineRounded, width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::Help), }
                WelcomeCard { label: "Logs", icon: rsx!{Icon{data: lucide::Logs, width:"20px", height:"20px"}}, onclick: move |_| nav(TabContent::Logs), }
            }
        }
    }
}

#[component]
fn WelcomeCard(label: &'static str, icon: Element, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div{
            class: "welcome-card",
            onclick: move |event| onclick.call(event),
            div{
                class: "welcome-card-icon",
                {icon}
            }
            span{
                class: "welcome-card-label",
                "{label}"
            }
        }
    }
}
