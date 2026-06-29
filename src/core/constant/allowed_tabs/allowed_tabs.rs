use crate::core::enums::{permissions::Permission, tab_content::TabContent};

pub fn allowed_tabs(permission: &Permission) -> Vec<TabContent> {
    match permission {
        Permission::Guest => vec![TabContent::Welcome, TabContent::Help],
        Permission::User => vec![
            TabContent::Welcome,
            TabContent::Help,
            TabContent::Home,
            TabContent::Dashboard,
            TabContent::FileExplorer(String::new()),
            TabContent::GMail,
            TabContent::GDrive,
            TabContent::ICloud,
        ],
        Permission::Admin => vec![
            TabContent::Welcome,
            TabContent::Help,
            TabContent::Home,
            TabContent::Dashboard,
            TabContent::FileExplorer(String::new()),
            TabContent::Editor(String::new()),
            TabContent::GMail,
            TabContent::GDrive,
            TabContent::ICloud,
            TabContent::Settings,
            TabContent::Terminal,
            TabContent::Account,
        ],
    }
}

pub fn is_tab_allowed(permission: &Permission, content: &TabContent) -> bool {
    allowed_tabs(permission)
        .iter()
        .any(|allowed| std::mem::discriminant(allowed) == std::mem::discriminant(content))
}
