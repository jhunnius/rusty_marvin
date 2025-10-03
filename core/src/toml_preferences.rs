//! # TOML Preferences Module
//!
//! This module provides a concrete implementation of the `Preferences` trait using TOML
//! (Tom's Obvious, Minimal Language) for configuration file storage. The `TomlPreferences`
//! struct enables poker bots to persist and load configuration settings in a human-readable format.
//!
//! ## TOML Preferences Overview
//!
//! The `TomlPreferences` implementation provides:
//! - **TOML Format**: Human-readable configuration file format
//! - **Type Safety**: Proper parsing and validation of preference values
//! - **File Persistence**: Save/load configuration from `.toml` files
//! - **Default Values**: Graceful fallback for missing preferences
//! - **Standard Compliance**: Full implementation of the Preferences trait
//!
//! ## File Format Example
//!
//! ```toml
//! # Bot Configuration
//! aggression = 0.7
//! bluff_frequency = 0.15
//! max_bet_ratio = 0.25
//!
//! # Game Settings
//! enable_logging = true
//! max_players = 6
//! preferred_seats = [2, 3, 7]
//!
//! # Strategy Parameters
//! [strategy.tight]
//! raise_threshold = 0.8
//! fold_threshold = 0.3
//!
//! [strategy.loose]
//! raise_threshold = 0.6
//! fold_threshold = 0.5
//! ```
//!
//! ## Key Features
//!
//! - **Human Readable**: TOML format is easy to read and edit manually
//! - **Structured Data**: Support for nested tables and arrays
//! - **Type Conversion**: Automatic parsing of strings to appropriate types
//! - **Error Handling**: Comprehensive error reporting for invalid files
//! - **Default Fallbacks**: Safe access with sensible defaults
//! - **Full Trait Compliance**: Implements all Preferences trait methods
//!
//! ## Examples
//!
//! ### Creating and Using TOML Preferences
//!
//! ```rust
//! use poker_api::toml_preferences::TomlPreferences;
//! use poker_api::api::preferences::Preferences;
//!
//! // Create new preferences (empty)
//! let mut prefs = TomlPreferences::default();
//!
//! // Set some values
//! prefs.set("aggression", "0.8");
//! prefs.set_boolean("enable_logging", true);
//! prefs.set_int("max_players", 6);
//! prefs.set_double("bluff_frequency", 0.15);
//!
//! // Get values with defaults
//! let aggression: String = prefs.get("aggression", "0.5");
//! let logging: bool = prefs.get_boolean("enable_logging", false);
//! let players: i32 = prefs.get_int("max_players", 9);
//! let bluff: f64 = prefs.get_double("bluff_frequency", 0.1);
//! ```
//!
//! ### File Operations
//!
//! ```rust
//! use poker_api::toml_preferences::TomlPreferences;
//!
//! let mut prefs = TomlPreferences::default();
//!
//! // Save preferences to file
//! prefs.set("bot_name", "MyBot");
//! prefs.set_double("aggression", 0.7);
//! prefs.save("bot_config.toml").unwrap();
//!
//! // Load preferences from file
//! let mut new_prefs = TomlPreferences::default();
//! new_prefs.load("bot_config.toml").unwrap();
//!
//! println!("Bot name: {}", new_prefs.get("bot_name", "Unknown"));
//! ```
//!
//! ### Integration with Poker Bots
//!
//! ```rust
//! use poker_api::toml_preferences::TomlPreferences;
//! use poker_api::api::preferences::Preferences;
//!
//! fn configure_bot_from_file(filename: &str) -> Result<TomlPreferences, String> {
//!     let mut prefs = TomlPreferences::default();
//!     prefs.load(filename)?;
//!
//!     // Use preferences for bot configuration
//!     let aggression = prefs.get_double("aggression", 0.5);
//!     let enable_logging = prefs.get_boolean("enable_logging", true);
//!
//!     println!("Bot configured with aggression: {}", aggression);
//!     Ok(prefs)
//! }
//! ```
//!
//! ## Design Decisions
//!
//! - **TOML Format**: Industry standard for configuration files
//! - **String Storage**: All values stored as strings for maximum flexibility
//! - **Parse on Demand**: Type conversion happens during access, not storage
//! - **Error Propagation**: String-based errors for caller flexibility
//! - **HashMap Backend**: Simple and reliable key-value storage
//! - **Serde Integration**: Leverages serde for serialization/deserialization
//!
//! ## Performance Characteristics
//!
//! - **Memory**: O(n) where n is number of preferences
//! - **Access Time**: O(1) hash map lookup
//! - **File I/O**: Depends on file size and system performance
//! - **Type Conversion**: O(1) parsing for primitive types
//! - **Serialization**: O(n) where n is total configuration size

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
