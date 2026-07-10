#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiskImageType {
    Iso,
    Dmg,
    Img,
    Vhd,
    Vhdx,
    Vmdk,
}
