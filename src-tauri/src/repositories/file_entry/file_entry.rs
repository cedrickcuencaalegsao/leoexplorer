#![allow(non_snake_case)]
use crate::core::enums::{
    cloud_status::CloudStatus, explorer_state::ExplorerState, flag::Flag,
    fs_attribute::FsAttribute, item_permissions::ItemPermissions,
};
use crate::core::models::file_entry::FileEntry;
use std::fs::*;
#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

impl FileEntry {
    #[allow(dead_code)]
    pub fn FromPath(path: &Path, is_dir: bool) -> Self {
        let path_str: String = Self::NormalizePath(path);

        let name: String = path
            .file_name()
            .and_then(|f| f.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                path.iter()
                    .last()
                    .and_then(|f| f.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| path_str.clone())
            });

        let entry_type: String = if is_dir {
            "folder".to_string()
        } else {
            "file".to_string()
        };

        let file_type: Option<String> = if !is_dir {
            path.extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase())
        } else {
            None
        };

        let metadata_result: Option<Metadata> = metadata(path).ok();

        let size: Option<u64> = if !is_dir {
            metadata_result.as_ref().map(|m| m.len())
        } else {
            None
        };

        let date_modified: Option<String> = metadata_result
            .as_ref()
            .and_then(|m| m.modified().ok())
            .and_then(|t| Self::FormatSystemTime(t));

        let date_created: Option<String> = metadata_result
            .as_ref()
            .and_then(|m| m.created().ok())
            .and_then(|t| Self::FormatSystemTime(t));

        let flag: Vec<Flag> = Self::CollectFlags(path, is_dir, &metadata_result);

        Self {
            name,
            path: path_str,
            is_dir,
            entry_type,
            file_type,
            size,
            flag,
            date_modified,
            date_created,
            children: None,
        }
    }

    fn CollectFlags(path: &Path, is_dir: bool, metadata: &Option<Metadata>) -> Vec<Flag> {
        let mut flags: Vec<Flag> = Vec::new();

        Self::CollectAttributes(path, metadata, &mut flags);
        Self::CollectItemPermissions(path, metadata, &mut flags);
        Self::CollectExplorerState(path, is_dir, &mut flags);
        Self::CollectCloudStatus(path, &mut flags);

        flags
    }

    fn CollectAttributes(_path: &Path, metadata: &Option<Metadata>, flag: &mut Vec<Flag>) {
        #[cfg(target_os = "linux")]
        Self::CollectLinuxiFlag(_path, flag);
        #[cfg(target_os = "windows")]
        Self::CollectWindowsAttributes(metadata, flag);
        #[cfg(target_os = "macos")]
        Self::CollectUnixAttributes(_path, metadata, flag);
    }

    #[cfg(target_os = "windows")]
    fn CollectWindowsAttributes(metadata: &Option<Metadata>, flag: &mut Vec<Flag>) {
        const FILE_ATTRIBUTE_READONLY: u32 = 0x0000_0001;
        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x0000_0002;
        const FILE_ATTRIBUTE_SYSTEM: u32 = 0x0000_0004;
        const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x0000_0020;
        const FILE_ATTRIBUTE_COMPRESSED: u32 = 0x0000_0080;
        const FILE_ATTRIBUTE_ENCRYPTED: u32 = 0x0000_4000;
        const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: u32 = 0x0000_8000;

        if let Some(meta) = metadata {
            let attrs = meta.file_attributes();

            if attrs & FILE_ATTRIBUTE_HIDDEN != 0 {
                flags.push(Flag::Attribute(FsAttribute::Hidden));
            }
            if attrs & FILE_ATTRIBUTE_READONLY != 0 {
                flags.push(Flag::Attribute(FsAttribute::ReadOnly));
            }
            if attrs & FILE_ATTRIBUTE_SYSTEM != 0 {
                flags.push(Flag::Attribute(FsAttribute::System));
            }
            if attrs & FILE_ATTRIBUTE_ARCHIVE != 0 {
                flags.push(Flag::Attribute(FsAttribute::Archive));
            }
            if attrs & FILE_ATTRIBUTE_NOT_INDEXED == 0 {
                // Bit is clear → indexing is enabled
                flags.push(Flag::Attribute(FsAttribute::Indexed));
            }
            if attrs & FILE_ATTRIBUTE_COMPRESSED != 0 {
                flags.push(Flag::Attribute(FsAttribute::Compressed));
            }
            if attrs & FILE_ATTRIBUTE_ENCRYPTED != 0 {
                flags.push(Flag::Attribute(FsAttribute::Encrypted));
            }
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn CollectUnixAttributes(path: &Path, metadata: &Option<Metadata>, flags: &mut Vec<Flag>) {
        let mut hidden = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with('.'))
            .unwrap_or(false);

        #[cfg(target_os = "macos")]
        if !hidden {
            hidden = Self::macOSHasUfHidden(path);
        }

        if hidden {
            flags.push(Flag::Attribute(FsAttribute::Hidden));
        }

        if let Some(meta) = metadata {
            let mode = meta.mode();
            if mode & 0o222 == 0 {
                flags.push(Flag::Attribute(FsAttribute::ReadOnly));
            }
        }

        #[cfg(target_os = "macos")]
        Self::macOSCollectXAttr(path, flags);

        #[cfg(target_os = "linux")]
        Self::CollectLinuxiFlag(path, flags);
    }

    fn macOSHasUfHidden(path: &Path) -> bool {
        use std::ffi::CString;
        use std::os::unix::ffi::OsStrExt;

        const UF_HIDDEN: u32 = 0x0000_8000;

        let c_path = match CString::new(path.as_os_str().as_bytes()) {
            Ok(p) => p,
            Err(_) => return false,
        };

        unsafe {
            let mut st: libc::stat = std::mem::zeroed();
            if libc::stat(c_path.as_ptr(), &mut st as *mut libc::stat) != 0 {
                return false;
            }
            (st.st_flags as u32) & UF_HIDDEN != 0
        }
    }

    fn macOSCollectXAttr(path: &Path, flags: &mut Vec<Flag>) {
        use std::process::Command;

        let path_str = path.to_string_lossy().to_string();

        if let Ok(output) = Command::new("ls").args(["-lOd", &path_str]).output() {
            let out = String::from_utf8_lossy(&output.stdout);
            if out.contains("schg") || out.contains("uchg") {
                flags.push(Flag::Attribute(FsAttribute::Immutable));
            }
            if out.contains("uappnd") || out.contains("sappnd") {
                flags.push(Flag::Attribute(FsAttribute::AppendOnly));
            }
        }

        if let Ok(output) = Command::new("xattr").args(["-l", &path_str]).output() {
            let out = String::from_utf8_lossy(&output.stdout);
            if out.contains("com.apple.cprotect") {
                flags.push(Flag::Attribute(FsAttribute::Encrypted));
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn CollectLinuxiFlag(path: &Path, flags: &mut Vec<Flag>) {
        use std::fs::File;
        use std::os::unix::io::AsRawFd;

        const FS_IOC_GETFLAGS: u64 = 0x8004_6601;
        const FS_IMMUTABLE_FL: u32 = 0x0000_0010;
        const FS_APPEND_FL: u32 = 0x0000_0020;
        const FS_COMPR_FL: u32 = 0x0000_0004;
        const FS_ENCRYPT_FL: u32 = 0x0000_0800;

        if let Ok(file) = File::open(path) {
            let fd = file.as_raw_fd();
            let mut iflags: u32 = 0;
            let ret = unsafe { libc::ioctl(fd, FS_IOC_GETFLAGS, &mut iflags as *mut u32) };

            if ret == 0 {
                if iflags & FS_IMMUTABLE_FL != 0 {
                    flags.push(Flag::Attribute(FsAttribute::Immutable));
                }
                if iflags & FS_APPEND_FL != 0 {
                    flags.push(Flag::Attribute(FsAttribute::AppendOnly));
                }
                if iflags & FS_COMPR_FL != 0 {
                    flags.push(Flag::Attribute(FsAttribute::Compressed));
                }
                if iflags & FS_ENCRYPT_FL != 0 {
                    flags.push(Flag::Attribute(FsAttribute::Encrypted));
                }
            }
        }
    }

    fn CollectItemPermissions(_path: &Path, metadata: &Option<Metadata>, flag: &mut Vec<Flag>) {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::fs::MetadataExt;
            const FILE_ATTRIBUTE_READONLY: u32 = 0x0000_0001;

            if let Some(meta) = metadata {
                let attrs = meta.file_attributes();
                flags.push(Flag::Permission(Permissions::Read));
                if attrs & FILE_ATTRIBUTE_READONLY == 0 {
                    flags.push(Flag::Permission(Permissions::Write));
                    flags.push(Flag::Permission(Permissions::FullControl));
                }
            }
        }
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            use std::os::unix::fs::MetadataExt;

            if let Some(meta) = metadata {
                let mode = meta.mode();

                let owner_read = 0o400u32;
                let owner_write = 0o200u32;
                let owner_execute = 0o100u32;
                let all_rwx = owner_read | owner_write | owner_execute;

                if mode & owner_read != 0 {
                    flag.push(Flag::ItemPermission(ItemPermissions::Read));
                }
                if mode & owner_write != 0 {
                    flag.push(Flag::ItemPermission(ItemPermissions::Write));
                }
                if mode & owner_execute != 0 {
                    flag.push(Flag::ItemPermission(ItemPermissions::Execute));
                }
                if mode & all_rwx == all_rwx {
                    flag.push(Flag::ItemPermission(ItemPermissions::FullControl));
                }
            }
        }
    }

    fn CollectExplorerState(path: &Path, is_dir: bool, flag: &mut Vec<Flag>) {
        if symlink_metadata(path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
        {
            flag.push(Flag::Explorer(ExplorerState::Shortcut));
        }

        #[cfg(any(target_os = "linux", target_os = "macos"))]
        if is_dir {
            Self::DetectUnixMountPoint(path, flag);
        }

        #[cfg(target_os = "windows")]
        if is_dir {
            Self::DetectWindowsMountPoint(path, flag);
        }

        #[cfg(target_os = "linux")]
        if is_dir {
            if let Ok(meta) = fs::metadata(path) {
                let mode = meta.mode();
                if mode & 0o020 != 0 || mode & 0o002 != 0 {
                    flag.push(Flag::Explorer(ExplorerState::Shared));
                }
            }
        }

        #[cfg(target_os = "windows")]
        if is_dir {
            let desktop_ini = path.join("desktop.ini");
            if let Ok(content) = fs::read_to_string(&desktop_ini) {
                if content.contains("PinToNameSpaceTree") {
                    flag.push(Flag::Explorer(ExplorerState::Pinned));
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let path_str = path.to_string_lossy().to_string();
            if let Ok(output) = Command::new("xattr").args(["-l", &path_str]).output() {
                let out = String::from_utf8_lossy(&output.stdout);
                if out.contains("com.apple.ubiquity-inode-identifier")
                    && !out.contains("com.apple.icloud.itemName")
                {
                    flag.push(Flag::Explorer(ExplorerState::OfflineAvailable));
                }
            }
        }
    }

    fn DetectUnixMountPoint(path: &Path, flags: &mut Vec<Flag>) {
        if let (Ok(self_meta), Some(parent)) = (metadata(path), path.parent()) {
            if let Ok(parent_meta) = metadata(parent) {
                if self_meta.dev() != parent_meta.dev() {
                    flags.push(Flag::Explorer(ExplorerState::MountPoint));
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn DetectWindowsMountPoint(path: &Path, flag: &mut Vec<Flag>) {
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;

        if let Ok(meta) = metadata(path) {
            if meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                flag.push(Flag::Explorer(ExplorerState::MountPoint));
            }
        }
    }

    fn CollectCloudStatus(_path: &Path, flags: &mut Vec<Flag>) {
        #[cfg(target_os = "windows")]
        Self::collect_cloud_status_windows(_path, flags);

        #[cfg(target_os = "macos")]
        Self::MacOSCollectCloudStatus(_path, flags);

        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        flags.push(Flag::Cloud(CloudStatus::Local));
    }

    #[cfg(target_os = "windows")]
    fn WindowsCollectCloudStatus(path: &Path, flags: &mut Vec<Flag>) {
        const RECALL_ON_DATA_ACCESS: u32 = 0x0040_0000;
        const RECALL_ON_OPEN: u32 = 0x0004_0000;
        const PINNED: u32 = 0x0008_0000;
        const UNPINNED: u32 = 0x0010_0000;

        if let Ok(meta) = fs::metadata(path) {
            let attrs = meta.file_attributes();

            if attrs & RECALL_ON_DATA_ACCESS != 0 || attrs & RECALL_ON_OPEN != 0 {
                flags.push(Flag::Cloud(CloudStatus::OnlineOnly));
            } else if attrs & PINNED != 0 {
                flags.push(Flag::Cloud(CloudStatus::Local));
            } else if attrs & UNPINNED != 0 {
                flags.push(Flag::Cloud(CloudStatus::OnlineOnly));
            } else {
                flags.push(Flag::Cloud(CloudStatus::Local));
            }
        } else {
            flags.push(Flag::Cloud(CloudStatus::Local));
        }
    }

    #[cfg(target_os = "macos")]
    fn MacOSCollectCloudStatus(path: &Path, flag: &mut Vec<Flag>) {
        use std::process::Command;

        let path_str = path.to_string_lossy().to_string();

        if let Ok(output) = Command::new("xattr").args(["-l", &path_str]).output() {
            let out = String::from_utf8_lossy(&output.stdout);

            if out.contains("com.apple.icloud.itemName") {
                flag.push(Flag::Cloud(CloudStatus::OnlineOnly));
            } else if out.contains("com.apple.ubiquity-inode-identifier") {
                flag.push(Flag::Cloud(CloudStatus::Local));
            } else {
                flag.push(Flag::Cloud(CloudStatus::Local));
            }
        } else {
            flag.push(Flag::Cloud(CloudStatus::Local));
        }
    }

    #[cfg(target_os = "linux")]
    fn LinuxCollectCloudStatus() {}

    pub fn FormatSystemTime(time: SystemTime) -> Option<String> {
        use chrono::{DateTime, Local};

        match time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                let datetime = DateTime::<Local>::from(SystemTime::UNIX_EPOCH + duration);
                Some(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
            }
            Err(_) => None,
        }
    }

    pub fn NormalizePath(path: &Path) -> String {
        let path_str = path.to_string_lossy().to_string();

        if cfg!(windows) {
            path_str.replace("\\", "/")
        } else {
            path_str
        }
    }

    #[allow(dead_code)]
    pub fn ToPathbuf(&self) -> PathBuf {
        if cfg!(windows) {
            PathBuf::from(self.path.replace("/", "\\"))
        } else {
            PathBuf::from(&self.path)
        }
    }

    #[allow(dead_code)]
    pub fn AbsolutePath(&self) -> std::io::Result<PathBuf> {
        let path = self.ToPathbuf();
        path.canonicalize()
    }

    #[allow(dead_code)]
    pub fn Exits(&self) -> bool {
        self.ToPathbuf().exists()
    }

    #[allow(dead_code)]
    pub fn New(
        name: String,
        path: String,
        is_dir: bool,
        file_type: Option<String>,
        size: Option<u64>,
        date_modified: Option<String>,
        date_created: Option<String>,
        children: Option<Vec<FileEntry>>,
    ) -> Self {
        let entry_type: String = if is_dir {
            "folder".to_string()
        } else {
            "file".to_string()
        };

        FileEntry {
            name,
            path,
            is_dir,
            entry_type,
            file_type,
            size,
            flag: Vec::new(),
            date_modified,
            date_created,
            children,
        }
    }
}
