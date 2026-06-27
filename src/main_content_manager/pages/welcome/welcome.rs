use dioxus::prelude::*;

#[component]
pub fn Welcome() -> Element {
    rsx! {
        div{
            h1 { "Welcome" }
        }
    }
}
