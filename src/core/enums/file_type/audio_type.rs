use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioType {
    Mp3,
    Wav,
    Flac,
    Aac,
    Ogg,
    Opus,
    M4a,
    Midi,
}
