use crate::icons::*;
use crate::shared::design::design::{
    linux_control_style, macos_control_style, windows_control_style,
};
#[allow(non_snake_case)]
use dioxus::prelude::*;
use serde_json::*;

#[component]
pub fn WindowControl() -> Element {
    let is_maximized = use_signal(|| false);
    let is_windows = use_signal(|| false);
    let is_linux = use_signal(|| false);
    let is_macos = use_signal(|| true);

    let controls_style = if is_macos() {
        macos_control_style()
    } else if is_windows() {
        windows_control_style()
    } else if is_linux() {
        linux_control_style()
    } else {
        String::new()
    };

    rsx! {
        style { "{controls_style}" }
        div {
            class: "window-controls",
            if is_macos() {
                CloseButton {  }
                MinimizeButton {  }
                RestoreButton { is_maximized }
            } else {
                MinimizeButton{},
                RestoreButton{is_maximized},
                CloseButton {}
            }
        }
    }
}

#[component]
fn MinimizeButton() -> Element {
    rsx! {
        button {
            class: "traffic-minimize",
            onclick: move |_| {
                document::eval(
                    "window.__TAURI__.window.getCurrentWindow().minimize()"
                );
            },
            Icon{data: material_symbols::MinimizeRounded, width:20, height:20}
        }
    }
}

#[component]
fn RestoreButton(mut is_maximized: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "traffic-restore",
            onclick: move |_| {
                let mut is_max = is_maximized;
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
    }
}

#[component]
fn CloseButton() -> Element {
    rsx! {
        button {
            class: "traffic-close",
            onclick: move |_| {
                document::eval(
                    "window.__TAURI__.window.getCurrentWindow().close()"
                );
            },
            Icon{data: material_symbols::CloseRounded, width:20, height:20}
        }
    }
}
