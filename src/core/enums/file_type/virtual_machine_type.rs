#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirtualMachineType {
    Vdi,
    Vmdk,
    Vhd,
    Vhdx,
    Qcow2,
}
