use crate::core::enums::file_type::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    Folder,
    Unknown,

    // Common Categories
    Document(DocumentType),
    Image(ImageType),
    Video(VideoType),
    Audio(AudioType),
    Archive(ArchiveType),
    Executable(ExecutableType),
    Developer(DeveloperType),
    Database(DatabaseType),
    Font(FontType),
    Config(ConfigType),
    System(SystemType),
    DiskImage(DiskImageType),
    Shortcut(ShortcutType),
    VirtualMachine(VirtualMachineType),
    Backup(BackupType),
}
