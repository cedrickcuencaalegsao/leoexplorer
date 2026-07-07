use crate::core::constant::CACHE_FILE_NAME;
use std::path::PathBuf;

pub struct PathService;

impl Default for PathService {
    fn default() -> Self {
        PathService
    }
}

impl PathService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base_path(&self) -> PathBuf {
        let base_path = if let Ok(home) = std::env::var("USERPROFILE") {
            // Windows
            PathBuf::from(home)
        } else if let Ok(home) = std::env::var("HOME") {
            // Unix-like
            PathBuf::from(home)
        } else {
            PathBuf::from(".")
        };
        base_path.join(".leo_explorer")
    }

    pub fn cache_path(&self) -> PathBuf {
        self.base_path().join("cache")
    }

    pub fn settings_path(&self) -> PathBuf {
        self.base_path().join("settings").join("settings.json")
    }

    pub fn find_cache(&self) -> bool {
        let cache_path = self.cache_path();
        if !cache_path.exists() {
            return false;
        }
        cache_path.join(CACHE_FILE_NAME).exists()
    }
}

// pub fn base_path() -> PathBuf {
//     PathService::new().base_path()
// }

// pub fn cache_path() -> PathBuf {
//     PathService::new().cache_path()
// }

// pub fn settings_path() -> PathBuf {
//     PathService::new().settings_path()
// }

// pub fn find_cache() -> bool {
//     PathService::new().find_cache()
// }
