#![allow(non_snake_case)]
use crate::core::enums::flag::Flag;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub entry_type: String,        // file or folder
    pub file_type: Option<String>, // e.g., txt, jpg
    pub size: Option<u64>,
    pub flag: Vec<Flag>, // Flags for the file entry
    pub date_modified: Option<String>,
    pub date_created: Option<String>,
    pub children: Option<Vec<FileEntry>>,
}
