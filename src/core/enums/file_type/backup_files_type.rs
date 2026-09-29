use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackupType {
    Backup,
    Bak,
    Old,
    Tmp,
}
