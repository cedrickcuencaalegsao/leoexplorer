#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExplorerState {
    Pinned,
    Shared,
    OfflineAvailable,
    Shortcut,
    MountPoint,
}
