#[allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::window_control::WindowControl;
use crate::shared::{
    design::design::title_bar_style,
    models::{app_state::AppState, tab::Tab},
};

#[component]
pub fn TitleBar() -> Element {
    let state = use_context::<AppState>();
    let mut tabs = state.tabs;
    let mut active = state.active_tab;
    let tab_list = tabs.read().clone();

    rsx! {
        style { "{title_bar_style()}" }
        div{

            class: "title-bar",
            "data-tauri-drag-region": "true",

            div{
                class: "tab-strip",
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
                                tabs.write().retain(|t| t.id != tab.id);
                            },
                            span{
                                "x"
                            }
                        }
                    }
                }
                button {
                    class: "tab-new",
                    onmousedown: move |event| { event.stop_propagation(); },
                    onclick: move |_| {
                        let new_id = tabs.read().len();
                        tabs.write().push(
                            Tab{
                                id: new_id,
                                title: "new tab".into(), path: "".into()
                            }
                        );
                    },
                    span{
                        "+"
                    }
                }
            },
            div {
                class:"title-bar-spacer"
            },
            WindowControl {  }
        }
    }
}
