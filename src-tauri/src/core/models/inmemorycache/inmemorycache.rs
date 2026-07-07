use crate::core::models::file_entry::FileEntry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InMemoryCache {
    pub roots: HashMap<PathBuf, FileEntry>,
}

impl InMemoryCache {
    pub fn new() -> Self {
        Self {
            roots: HashMap::new(),
        }
    }
}

impl Default for InMemoryCache {
    fn default() -> Self {
        Self::new()
    }
}
