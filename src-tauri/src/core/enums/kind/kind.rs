use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Kind {
    Image,
    Video,
    Audio,
    Document,
    Code,
    Archive,
}
