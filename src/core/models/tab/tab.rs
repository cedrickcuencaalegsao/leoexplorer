use crate::core::enums::tab_content::TabContent;

#[derive(Clone, PartialEq)]
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub content: TabContent,
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::new(),
            content: TabContent::Welcome,
        }
    }
}
