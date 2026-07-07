use crate::core::models::file_entry::FileEntry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CacheData {
    pub roots: Vec<FileEntry>,
}
