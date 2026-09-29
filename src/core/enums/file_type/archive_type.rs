use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchiveType {
    Zip,
    Rar,
    SevenZip,
    Tar,
    Gzip,
    Bzip2,
    Xz,
    Iso,
}
