use crate::core::models::inmemorycache::InMemoryCache;
use std::sync::Mutex;

pub struct AppCacheState {
    pub data: Mutex<InMemoryCache>,
}
