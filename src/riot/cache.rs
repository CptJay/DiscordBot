use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
pub struct Cache {
    pub user_puuid: HashMap<String, String>, // key: username#tag, value: puuid
                                             // Add more cache maps here as needed
}

impl Cache {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        if let Ok(data) = fs::read_to_string(&path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Cache::default()
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    pub fn get_user_puuid(&self, username: &str, tag: &str) -> Option<&String> {
        let key = format!("{}#{}", username, tag);
        self.user_puuid.get(&key)
    }

    pub fn insert_user_puuid(&mut self, username: &str, tag: &str, puuid: &str) {
        let key = format!("{}#{}", username, tag);
        self.user_puuid.insert(key, puuid.to_string());
    }
    // Add more generic cache methods as needed
}
