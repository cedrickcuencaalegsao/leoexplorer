use crate::core::enums::file_type::FileType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    #[allow(dead_code)]
    File(FileType),
    #[allow(dead_code)]
    Folder,
    #[allow(dead_code)]
    Unknown,
    #[allow(dead_code)]
    Default,
}
