use crate::api::preferences::Preferences;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use toml::{from_str, to_string};

#[derive(Serialize, Deserialize, Default)]
pub struct TomlPreferences {
    preferences: HashMap<String, String>,
}

impl Preferences for TomlPreferences {
    fn get(&self, name: &str, default: &str) -> String {
        self.preferences
            .get(name)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    fn get_boolean(&self, name: &str, default: bool) -> bool {
        self.preferences
            .get(name)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn get_int(&self, name: &str, default: i32) -> i32 {
        self.preferences
            .get(name)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn get_double(&self, name: &str, default: f64) -> f64 {
        self.preferences
            .get(name)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    fn set(&mut self, name: &str, value: &str) {
        self.preferences.insert(name.to_string(), value.to_string());
    }

    fn set_boolean(&mut self, name: &str, value: bool) {
        self.preferences.insert(name.to_string(), value.to_string());
    }

    fn set_int(&mut self, name: &str, value: i32) {
        self.preferences.insert(name.to_string(), value.to_string());
    }

    fn set_double(&mut self, name: &str, value: f64) {
        self.preferences.insert(name.to_string(), value.to_string());
    }

    fn has_preference(&self, name: &str) -> bool {
        self.preferences.contains_key(name)
    }

    fn remove_preference(&mut self, name: &str) {
        self.preferences.remove(name);
    }

    fn save(&self, file_path: &str) -> Result<(), String> {
        let toml_str = to_string(self).map_err(|e| e.to_string())?;
        fs::write(file_path, toml_str).map_err(|e| e.to_string())
    }

    fn load(&mut self, file_path: &str) -> Result<(), String> {
        let toml_str = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
        *self = from_str(&toml_str).map_err(|e| e.to_string())?;
        Ok(())
    }
}
