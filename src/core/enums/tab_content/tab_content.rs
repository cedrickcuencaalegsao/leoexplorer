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
    Home,
    Logs,
}

impl TabContent {
    pub fn title(&self) -> &'static str {
        match self {
            TabContent::Welcome => "Welcome",
            TabContent::FileExplorer(_) => "File Explorer",
            TabContent::Editor(_) => "Editor",
            TabContent::Settings => "Settings",
            TabContent::Terminal => "Terminal",
            TabContent::Help => "Help",
            TabContent::Account => "Account",
            TabContent::GMail => "GMail",
            TabContent::GDrive => "GDrive",
            TabContent::ICloud => "ICloud",
            TabContent::Dashboard => "Dashboard",
            TabContent::Home => "Home",
            TabContent::Logs => "Logs",
        }
    }
}
