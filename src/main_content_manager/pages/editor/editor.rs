#![allow(non_snake_case)]
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct EditorProps {
    pub path: String,
}

pub fn Editor(props: EditorProps) -> Element {
    rsx! {
        div{
            p { "Editing: {props.path}" }
        }
    }
}
