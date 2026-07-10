#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutableType {
    Exe,
    Msi,
    App,
    Dmg,
    Apk,
    Deb,
    Rpm,
    Bin,
    Elf,
    Bat,
    Cmd,
    Sh,
    Ps1,
}
