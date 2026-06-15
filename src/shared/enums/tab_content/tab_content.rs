#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum TabContent {
    Welcome,
    FileExplorer(String), // path
    Editor(String),       // file path (this is usable for developer.)
    Settings,
    Terminal,
    Help,
    Account,
    GMail,
    GDrive,
    ICloud,
    Dashboard,
}
