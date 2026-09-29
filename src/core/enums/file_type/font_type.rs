use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontType {
    Ttf,
    Otf,
    Woff,
    Woff2,
}
