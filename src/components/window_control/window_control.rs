use crate::icons::*;
#[allow(non_snake_case)]
use dioxus::prelude::*;
use serde_json::*;

#[component]
pub fn WindowControl() -> Element {
    let is_maximized = use_signal(|| false);

    rsx! {
        div {
            class: "window-controls",
            button {
                onclick: move |_| {
                    document::eval(
                        "window.__TAURI__.window.getCurrentWindow().minimize()"
                    );
                },
                Icon{data: material_symbols::MinimizeRounded, width:20, height:20}
            },
            button {
                onclick: move |_| {
                    let mut is_max = is_maximized.clone();
                    async move {
                        let res = document::eval(
                            r#"
                                const win = window.__TAURI__.window.getCurrentWindow();
                                const max = await win.isMaximized();
                                if (max) { await win.unmaximize(); } else { await win.maximize(); }
                                return !max;
                            "#
                        ).await;
                        if let Ok(val) = res {
                            if let Ok(new_state) = from_value::<bool>(val) {
                                is_max.set(new_state);
                            }
                        }
                    }
                },
                if is_maximized() {
                    Icon{data: basil::ExpandOutline, width:20, height:20}
                } else {
                    Icon{data: basil::ExpandOutline, width:20, height:20}
                }
            },
            button {
                onclick: move |_| {
                    document::eval(
                        "window.__TAURI__.window.getCurrentWindow().close()"
                    );
                },
                Icon{data: material_symbols::CloseRounded, width:20, height:20}
            },
        }
    }
}
