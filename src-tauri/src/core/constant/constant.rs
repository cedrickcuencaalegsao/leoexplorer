use std::sync::atomic::AtomicU64;

pub const CACHE_FILE_NAME: &str = "leocache.bin.zst";
pub const CACHE_TEMP_FILE_NAME: &str = "leocache.bin.zst.tmp";
// pub static CACHE_SERVICE: Lazy<CacheService> = Lazy::new(CacheService::default);
pub static TREE_INSERT_SEQ: AtomicU64 = AtomicU64::new(0);
pub static TREE_INSERT_TOTAL: AtomicU64 = AtomicU64::new(0);
