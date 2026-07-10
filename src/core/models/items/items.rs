use crate::core::enums::{item_type::ItemType, view_mode::ViewMode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Items {
    pub name: String,
    pub item_type: ItemType,
    pub date_created: String,
    pub date_modified: String,
    pub flag: Option<String>,
    pub path: String,
    pub is_dir: bool,
    pub view_mode: ViewMode,
}
