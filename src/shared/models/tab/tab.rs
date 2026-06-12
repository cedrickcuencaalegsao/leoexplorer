use crate::shared::enums::tab_content::TabContent;

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub content: TabContent,
}
