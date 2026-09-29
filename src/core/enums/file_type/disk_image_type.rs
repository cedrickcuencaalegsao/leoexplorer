use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiskImageType {
    Iso,
    Dmg,
    Img,
    Vhd,
    Vhdx,
    Vmdk,
}
