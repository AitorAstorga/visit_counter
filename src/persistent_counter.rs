use std::collections::HashMap;
use std::sync::Mutex;
use std::path::Path;

/// A simple file‐based persistent storage for counters.
/// It loads data from a JSON file on initialization and writes changes
/// back to that file.
#[derive(Debug)]
pub struct PersistentCounterMap {
    data: Mutex<HashMap<String, u64>>,
    path: String,
}

impl PersistentCounterMap {
    /// Creates a new PersistentCounterMap.
    /// If the file at `path` exists, it loads the counter data from it.
    /// Otherwise, it starts with an empty map.
    pub fn new(path: &str) -> Self {
        let data = if Path::new(path).exists() {
            let content = std::fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };
        PersistentCounterMap {
            data: Mutex::new(data),
            path: path.to_string(),
        }
    }

    /// Gets the counter value for a given key.
    pub fn get(&self, key: &str) -> u64 {
        let data = self.data.lock().unwrap();
        *data.get(key).unwrap_or(&0)
    }

    /// Increments the counter for a given key, saves the change, and returns the new value.
    pub fn increment(&self, key: &str) -> u64 {
        // First, update the counter and capture the new value.
        let new_count = {
            let mut data = self.data.lock().unwrap();
            let count = data.entry(key.to_string()).or_insert(0);
            *count += 1;
            *count
        };
    
        // Now, re-lock the data and save it to disk.
        {
            let data = self.data.lock().unwrap();
            if let Ok(content) = serde_json::to_string_pretty(&*data) {
                let _ = std::fs::write(&self.path, content);
            }
        }
    
        new_count
    }
    
    /// Sets the counter for a given key to the specified value and saves the change.
    pub fn set(&self, key: &str, value: u64) {
        let mut data = self.data.lock().unwrap();
        data.insert(key.to_string(), value);
        if let Ok(content) = serde_json::to_string_pretty(&*data) {
            let _ = std::fs::write(&self.path, content);
        }
    }
}
