#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemPermissions {
    Read,
    Write,
    Execute,
    FullControl,
}
