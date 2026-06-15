use crate::components::side_panel::SidePanel;
use crate::shared::{enums::tab_content::TabContent, models::tab::Tab};
use dioxus::prelude::*;

#[component]
pub fn TabContentView(tab: Tab, is_active: bool) -> Element {
    match &tab.content {
        TabContent::GDrive => rsx! {
            CloudWebview {
                url: "https://drive.google.com",
                label: "gdrive",
                is_active,
            }
        },
        TabContent::ICloud => rsx! {
            CloudWebview {
                url: "https://www.icloud.com",
                label: "icloud",
                is_active,
            }
        },
        TabContent::GMail => rsx! {
            CloudWebview {
                url: "https://mail.google.com",
                label: "gmail",
                is_active,
            }
        },
        _ => rsx! {
            div {
                class: "app-container",
                div { class: "side-panel-container", SidePanel {} }
                div { class: "main-panel-container" }
                div { class: "preview-panel-container" }
                div { class: "dynamic-sidebar-container" }
            }
        },
    }
}

#[component]
fn CloudWebview(url: &'static str, label: &'static str, is_active: bool) -> Element {
    let mounted = use_signal(|| false);

    // Cleanup when component fully removed
    use_drop(move || {
        spawn(async move {
            let _ = document::eval(&format!(
                r#"window.__TAURI_INTERNALS__.invoke('close_cloud_view', {{ label: '{label}' }})"#
            ))
            .await;
        });
    });

    // First mount — create the child webview
    use_effect(move || {
        if !*mounted.read() {
            let script = format!(
                r#"
                (function embed() {{
                    const el = document.getElementById('cloud-anchor-{label}');
                    if (!el) {{ requestAnimationFrame(embed); return; }}
                    const r = el.getBoundingClientRect();
                    if (r.width === 0 || r.height === 0) {{ requestAnimationFrame(embed); return; }}

                    window.__TAURI_INTERNALS__.invoke('embed_cloud_view', {{
                        label:  '{label}',
                        url:    '{url}',
                        x:      Math.round(r.left),
                        y:      Math.round(r.top),
                        width:  Math.round(r.width),
                        height: Math.round(r.height),
                    }});

                    new ResizeObserver(() => {{
                        const r2 = el.getBoundingClientRect();
                        window.__TAURI_INTERNALS__.invoke('resize_cloud_view', {{
                            label:  '{label}',
                            x:      Math.round(r2.left),
                            y:      Math.round(r2.top),
                            width:  Math.round(r2.width),
                            height: Math.round(r2.height),
                        }});
                    }}).observe(el);
                }})();
            "#
            );
            spawn(async move {
                let _ = document::eval(&script).await;
            });
        }
    });

    // Show/hide when tab switches
    use_effect(move || {
        let action = if is_active {
            "show_cloud_view"
        } else {
            "hide_cloud_view"
        };
        let script =
            format!(r#"window.__TAURI_INTERNALS__.invoke('{action}', {{ label: '{label}' }})"#);
        spawn(async move {
            let _ = document::eval(&script).await;
        });
    });

    rsx! {
        div {
            id: "cloud-anchor-{label}",
            style: "width: 100%; height: 100%; display: block;",
        }
    }
}
