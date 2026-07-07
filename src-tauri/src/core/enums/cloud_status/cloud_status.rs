#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CloudStatus {
    Local,
    OnlineOnly,
    Syncing,
    Conflict,
    Error,
}
