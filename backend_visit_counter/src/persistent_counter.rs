// backend_visit_counter/src/persistent_counter.rs
use std::collections::HashMap;
use std::sync::Mutex;
use std::path::Path;
use chrono::Utc;
use crate::models::{Badge, BadgeResponse};

/// A simple file‐based persistent storage for counters.
/// It loads data from a JSON file on initialization and writes changes
/// back to that file.
#[derive(Debug)]
pub struct PersistentCounterMap {
    data: Mutex<HashMap<String, u64>>,
    badges: Mutex<HashMap<String, Badge>>,
    path: String,
    badges_path: String,
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

        let badges_path = path.replace(".json", "_badges.json");
        let badges = if Path::new(&badges_path).exists() {
            let content = std::fs::read_to_string(&badges_path).unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        PersistentCounterMap {
            data: Mutex::new(data),
            badges: Mutex::new(badges),
            path: path.to_string(),
            badges_path,
        }
    }

    /// Gets the counter value for a given key.
    pub fn get(&self, key: &str) -> u64 {
        let data = self.data.lock().unwrap();
        *data.get(key).unwrap_or(&0)
    }

    /// Increments the counter for a given key, saves the change, and returns the new value.
    pub fn increment(&self, key: &str) -> u64 {
        let now = Utc::now();

        // Update the counter and capture the new value.
        let new_count = {
            let mut data = self.data.lock().unwrap();
            let count = data.entry(key.to_string()).or_insert(0);
            *count += 1;
            *count
        };

        // Update badge metadata
        {
            let mut badges = self.badges.lock().unwrap();
            badges.entry(key.to_string()).and_modify(|badge| {
                badge.count = new_count;
                badge.last_accessed = now;
            }).or_insert_with(|| Badge {
                name: key.to_string(),
                count: new_count,
                created_at: now,
                last_accessed: now,
            });
        }

        // Save both files
        self.save_data();
        self.save_badges();

        new_count
    }
    
    /// Sets the counter for a given key to the specified value and saves the change.
    pub fn set(&self, key: &str, value: u64) {
        let now = Utc::now();

        {
            let mut data = self.data.lock().unwrap();
            data.insert(key.to_string(), value);
        }

        // Update badge metadata
        {
            let mut badges = self.badges.lock().unwrap();
            badges.entry(key.to_string()).and_modify(|badge| {
                badge.count = value;
                badge.last_accessed = now;
            }).or_insert_with(|| Badge {
                name: key.to_string(),
                count: value,
                created_at: now,
                last_accessed: now,
            });
        }

        self.save_data();
        self.save_badges();
    }

    /// Helper method to save counter data
    fn save_data(&self) {
        let data = self.data.lock().unwrap();
        if let Ok(content) = serde_json::to_string_pretty(&*data) {
            let _ = std::fs::write(&self.path, content);
        }
    }

    /// Helper method to save badge metadata
    fn save_badges(&self) {
        let badges = self.badges.lock().unwrap();
        if let Ok(content) = serde_json::to_string_pretty(&*badges) {
            let _ = std::fs::write(&self.badges_path, content);
        }
    }

    /// Get all badges for admin interface
    pub fn get_all_badges(&self) -> Vec<BadgeResponse> {
        let badges = self.badges.lock().unwrap();
        badges.values().map(|badge| BadgeResponse {
            name: badge.name.clone(),
            count: badge.count,
            created_at: badge.created_at,
            last_accessed: badge.last_accessed,
        }).collect()
    }

    /// Get a specific badge
    pub fn get_badge(&self, name: &str) -> Option<BadgeResponse> {
        let badges = self.badges.lock().unwrap();
        badges.get(name).map(|badge| BadgeResponse {
            name: badge.name.clone(),
            count: badge.count,
            created_at: badge.created_at,
            last_accessed: badge.last_accessed,
        })
    }

    /// Delete a badge and its counter
    pub fn delete_badge(&self, name: &str) -> bool {
        let mut data = self.data.lock().unwrap();
        let mut badges = self.badges.lock().unwrap();

        let removed_data = data.remove(name).is_some();
        let removed_badge = badges.remove(name).is_some();

        if removed_data || removed_badge {
            drop(data);
            drop(badges);
            self.save_data();
            self.save_badges();
            true
        } else {
            false
        }
    }

    /// Create a new badge with optional initial count
    pub fn create_badge(&self, name: &str, initial_count: Option<u64>) -> BadgeResponse {
        let now = Utc::now();
        let count = initial_count.unwrap_or(0);

        {
            let mut data = self.data.lock().unwrap();
            data.insert(name.to_string(), count);
        }

        let badge = Badge {
            name: name.to_string(),
            count,
            created_at: now,
            last_accessed: now,
        };

        {
            let mut badges = self.badges.lock().unwrap();
            badges.insert(name.to_string(), badge.clone());
        }

        self.save_data();
        self.save_badges();

        BadgeResponse {
            name: badge.name,
            count: badge.count,
            created_at: badge.created_at,
            last_accessed: badge.last_accessed,
        }
    }
}
