use crate::shared::models::folder_operation::FolderOperation;

pub struct FolderOperationSection {
    #[allow(dead_code)]
    pub title: &'static str, // "Basic Operations" / "Folder Management Operations"
    #[allow(dead_code)]
    pub section_class: &'static str, // "basic-operations" / "folder-management-operations"
    #[allow(dead_code)]
    pub item_class: &'static str, // "basic-operation-items" / "folder-management-operation-items"
    #[allow(dead_code)]
    pub icon_prefix: &'static str, // "boi" / "fm"
    #[allow(dead_code)]
    pub items: Vec<FolderOperation>,
}
