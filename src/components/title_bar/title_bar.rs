use crate::components::window_control::WindowControl;
use crate::icons::*;
use crate::core::{
    design::design::title_bar_style,
    enums::tab_content::TabContent,
    models::{app_state::AppState, tab::Tab},
};

#[allow(non_snake_case)]
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r#"
export function detectWindows() {
    return navigator.platform.toLowerCase().includes('win');
}
"#)]
extern "C" {
    fn detectWindows() -> bool;
}

#[component]
pub fn TitleBar() -> Element {
    let state = use_context::<AppState>();
    let mut tabs = state.tabs;
    let mut active = state.active_tab;
    let tab_list = tabs.read().clone();
    let mut is_windows = use_signal(|| false);

    use_effect(move || {
        is_windows.set(detectWindows());
    });

    rsx! {
        style { "{title_bar_style()}" }
        div{
            class: "title-bar",
            "data-tauri-drag-region": "true",
            if !is_windows() {
                div { class: "traffic-light-spacer" }
            }
            div{
                class: "tab-strip",
                "data-tauri-drag-region": "true",
                for tab in tab_list.into_iter() {
                    div{
                        key: "{tab.id}",
                        class: if *active.read() == tab.id { "tab active" } else { "tab" },
                        onclick: move |_| { active.set(tab.id); },
                        onmousedown: |event| event.stop_propagation(),
                        span{
                            "{tab.title}"
                        },
                        button{
                            class: "tab-close",
                            onclick: move |event| {
                                event.stop_propagation();
                                let closing_id = tab.id;
                                let was_active = *active.read() == closing_id;
                                let mut tabs_mut = tabs.write();
                                let pos = tabs_mut.iter().position(|t| t.id == closing_id);
                                tabs_mut.retain(|t| t.id != closing_id);
                                if was_active{
                                    if let Some(pos) = pos{
                                        let new_active = tabs_mut.get(pos).or_else(|| tabs_mut.last()).map(|t| t.id);
                                        if let Some(id) = new_active{
                                            active.set(id);
                                        }
                                    }
                                }
                            },
                            Icon{ data: material_symbols::CloseRounded, width: "12", height: "12" }
                        }
                    }
                }
                button {
                    class: "tab-new",
                    onmousedown: move |event| { event.stop_propagation(); },
                    onclick: move |_| {
                        let new_id = tabs.read().iter().map(|t| t.id).max().map(|id| id + 1).unwrap_or(0);
                        tabs.write().push(
                            Tab{
                                id: new_id,
                                title: "New tab".into(),
                                content: TabContent::Welcome,
                            }
                        );
                        active.set(new_id);
                    },
                    Icon{ data: material_symbols::AddRounded, width: "12", height: "12" }
                }
            },
            div {
                class:"title-bar-spacer"
            },
            if is_windows() {
                WindowControl {  }
            }
        }
    }
}
