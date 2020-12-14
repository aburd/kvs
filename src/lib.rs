#![deny(missing_docs)]
//! A simple key value store

use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
#[derive(Default)]
pub struct KvStore {
    data: HashMap<String, String>,
}

impl KvStore {
    /// A key-value store
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Sets a value in the store
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    /// Gets a value from the store
    pub fn get(&self, key: String) -> Option<String> {
        self.data.get(&key).cloned()
    }
    /// Removes a value from the store
    pub fn remove(&mut self, key: String) {
        self.data.remove(&key);
    }
}
