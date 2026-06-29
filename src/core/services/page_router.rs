#[allow(non_snake_case)]
use crate::core::enums::tab_content::TabContent;

pub struct PageRouter {
    pub page: TabContent,
}

impl PageRouter {
    pub fn new() -> Self {
        Self {
            page: TabContent::Welcome,
        }
    }

    pub fn navigate(&mut self, page: TabContent) {
        self.page = page;
    }
}

impl Default for PageRouter {
    fn default() -> Self {
        Self::new()
    }
}
