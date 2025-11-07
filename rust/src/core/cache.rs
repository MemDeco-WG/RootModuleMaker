//! Lightweight cache helpers (stub)
use std::time::Duration;

pub struct CacheItem<T> {
    pub data: T,
    pub ttl: Duration,
}

impl<T> CacheItem<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        CacheItem { data, ttl }
    }

    pub fn is_expired(&self) -> bool {
        false
    }
}
