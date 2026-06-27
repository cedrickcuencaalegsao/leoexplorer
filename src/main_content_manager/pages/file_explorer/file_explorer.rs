#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FileExplorerProps {
    pub path: String,
}

pub fn FileExplorer(props: FileExplorerProps) -> Element {
    rsx! {
        div{
            p{"file explorer"}
            p{"path: {props.path}"}
        }
    }
}
