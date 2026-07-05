#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FsAttribute {
    Hidden,
    ReadOnly,
    System,
    Archive,
    Indexed,
    Compressed,
    Encrypted,
    Immutable,
    AppendOnly,
}
