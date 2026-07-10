#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutType {
    WindowsShortcut,
    MacAlias,
    DesktopEntry,
}
