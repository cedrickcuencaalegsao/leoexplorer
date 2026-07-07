use crate::core::models::{
    cache::Cache, clipboard::Clipboard, deepsearchsettings::DeepSearchSettings,
    filemonitorignorelist::FileMonitorIgnoreList, pinfolder::pinfolder::PinFolder,
    systemsettings::SystemSettings,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub cache: Cache,
    pub deep_search: DeepSearchSettings,
    pub system_settings: SystemSettings,
    pub ignore_list: FileMonitorIgnoreList,
    pub clipboard: Clipboard,
    pub pin_folders: Vec<PinFolder>,
}
