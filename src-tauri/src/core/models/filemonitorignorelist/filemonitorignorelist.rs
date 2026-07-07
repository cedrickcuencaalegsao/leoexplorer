use crate::core::models::filemonitorignorelistlinux::FileMonitorIgnoreListLinux;
use crate::core::models::filemonitorignorelistmacos::FileMonitorIgnoreListMacOS;
use crate::core::models::filemonitorignorelistwindows::FileMonitorIgnoreListWindows;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct FileMonitorIgnoreList {
    pub windows: FileMonitorIgnoreListWindows,
    pub linux: FileMonitorIgnoreListLinux,
    pub macos: FileMonitorIgnoreListMacOS,
}
