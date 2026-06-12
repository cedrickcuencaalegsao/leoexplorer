#[allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn WindowControl() -> Element {
    rsx! {
        div {
            class: "window-controls",
            button {
                onclick: move |_| {
                    document::eval(
                        "window.__TAURI__.window.getCurrentWindow().minimize()"
                    );
                },
                "─"
            },
            button {
                onclick: move |_| {
                    document::eval(
                        r#"
                            const win = window.__TAURI__.window.getCurrentWindow();
                            win.isMaximized().then((isMaximized) => {
                                if (isMaximized) {
                                    win.unmaximize();
                                } else {
                                    win.maximize();
                                }
                            });
                        "#
                    );
                },
                "─"
            },
            button {
                onclick: move |_| {
                    document::eval(
                        "window.__TAURI__.window.getCurrentWindow().close()"
                    );
                },
                "x"
            },
        }
    }
}
