#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemType {
    Dll,
    Sys,
    Driver,
    Library,
    Cache,
    Temp,
    Log,
    Registry,
}
