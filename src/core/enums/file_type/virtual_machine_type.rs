use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VirtualMachineType {
    Vdi,
    Vmdk,
    Vhd,
    Vhdx,
    Qcow2,
}
