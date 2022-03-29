use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use log::debug;

#[derive(Debug, Default)]
pub struct SessionsCache {
    cache: Arc<Mutex<HashMap<String, i32>>>,
}

impl SessionsCache {
    pub fn insert(&self, key: String, value: i32) {
        debug!("added new session: {} - {}", key, value);
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<i32> {
        match self.cache.lock().unwrap().get(&key) {
            None => None,
            Some(v) => Some(*v),
        }
    }

    pub fn remove(&self, key: String) {
        let mut values = self.cache.lock().unwrap();
        values.remove(&key);
    }
}
