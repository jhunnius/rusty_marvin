//! # Preferences Module
//!
//! This module defines the `Preferences` trait for managing configuration and settings
//! for poker bots and game components. The trait provides a unified interface for
//! storing, retrieving, and persisting bot preferences and game configuration.
//!
//! ## Preferences Trait Overview
//!
//! The `Preferences` trait provides methods for:
//! - **Type-Safe Access**: Get/set values as strings, booleans, integers, or doubles
//! - **Default Values**: Provide fallback values for missing preferences
//! - **Persistence**: Save and load preferences from files
//! - **Validation**: Check for preference existence and remove unwanted settings
//!
//! ## Key Features
//!
//! - **Multiple Data Types**: Support for strings, booleans, integers, and floating-point values
//! - **Default Fallbacks**: Safe access with sensible defaults for missing preferences
//! - **File Persistence**: Save/load configuration from TOML or other formats
//! - **Dynamic Configuration**: Runtime preference modification and validation
//! - **Error Handling**: Graceful handling of missing or invalid preferences
//!
//! ## Supported Data Types
//!
//! - **String**: Text-based configuration values
//! - **Boolean**: True/false flags and options
//! - **Integer**: Numeric configuration (i32)
//! - **Double**: Floating-point configuration (f64)
//!
//! ## Examples
//!
//! ### Basic Preference Access
//!
//! ```rust
//! use poker_api::api::preferences::Preferences;
//!
//! struct MyBotPreferences {
//!     values: std::collections::HashMap<String, String>,
//! }
//!
//! impl Preferences for MyBotPreferences {
//!     fn get(&self, name: &str, default: &str) -> String {
//!         self.values.get(name).cloned().unwrap_or_else(|| default.to_string())
//!     }
//!
//!     fn get_boolean(&self, name: &str, default: bool) -> bool {
//!         self.values.get(name)
//!             .and_then(|s| s.parse().ok())
//!             .unwrap_or(default)
//!     }
//!
//!     fn get_int(&self, name: &str, default: i32) -> i32 {
//!         self.values.get(name)
//!             .and_then(|s| s.parse().ok())
//!             .unwrap_or(default)
//!     }
//!
//!     fn get_double(&self, name: &str, default: f64) -> f64 {
//!         self.values.get(name)
//!             .and_then(|s| s.parse().ok())
//!             .unwrap_or(default)
//!     }
//!
//!     fn set(&mut self, name: &str, value: &str) {
//!         self.values.insert(name.to_string(), value.to_string());
//!     }
//!
//!     fn set_boolean(&mut self, name: &str, value: bool) {
//!         self.values.insert(name.to_string(), value.to_string());
//!     }
//!
//!     fn set_int(&mut self, name: &str, value: i32) {
//!         self.values.insert(name.to_string(), value.to_string());
//!     }
//!
//!     fn set_double(&mut self, name: &str, value: f64) {
//!         self.values.insert(name.to_string(), value.to_string());
//!     }
//!
//!     fn has_preference(&self, name: &str) -> bool {
//!         self.values.contains_key(name)
//!     }
//!
//!     fn remove_preference(&mut self, name: &str) {
//!         self.values.remove(name);
//!     }
//!
//!     fn save(&self, _file_path: &str) -> Result<(), String> {
//!         // Implementation would save to file
//!         Ok(())
//!     }
//!
//!     fn load(&mut self, _file_path: &str) -> Result<(), String> {
//!         // Implementation would load from file
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ### Using Preferences in a Bot
//!
//! ```rust
//! use poker_api::api::preferences::Preferences;
//!
//! fn configure_bot(prefs: &dyn Preferences) {
//!     let aggression = prefs.get_double("aggression", 0.5);
//!     let bluff_frequency = prefs.get_double("bluff_frequency", 0.1);
//!     let max_bet_ratio = prefs.get_double("max_bet_ratio", 0.25);
//!
//!     let enable_logging = prefs.get_boolean("enable_logging", true);
//!     let max_players = prefs.get_int("max_players", 6);
//!
//!     println!("Bot configured with aggression: {}", aggression);
//! }
//! ```
//!
//! ## Design Decisions
//!
//! - **Trait-Based Design**: Enables different preference storage backends
//! - **Type-Safe Getters**: Separate methods for each data type with defaults
//! - **Mutable Setters**: Allow runtime configuration changes
//! - **File Persistence**: Standard save/load interface for all implementations
//! - **Error Handling**: String-based error returns for flexibility
//!
//! ## Performance Characteristics
//!
//! - **Access Time**: O(1) for hash map implementations
//! - **Type Conversion**: O(1) parsing for primitive types
//! - **File I/O**: Variable depending on implementation and file size
//! - **Memory**: Minimal overhead beyond storage backend

pub trait Preferences {
    /// Get a preference as a string, with a default value.
    fn get(&self, name: &str, default: &str) -> String;

    /// Get a preference as a boolean, with a default value.
    fn get_boolean(&self, name: &str, default: bool) -> bool;

    /// Get a preference as an integer, with a default value.
    fn get_int(&self, name: &str, default: i32) -> i32;

    /// Get a preference as a double, with a default value.
    fn get_double(&self, name: &str, default: f64) -> f64;

    /// Set a preference as a string.
    fn set(&mut self, name: &str, value: &str);

    /// Set a preference as a boolean.
    fn set_boolean(&mut self, name: &str, value: bool);

    /// Set a preference as an integer.
    fn set_int(&mut self, name: &str, value: i32);

    /// Set a preference as a double.
    fn set_double(&mut self, name: &str, value: f64);

    /// Check if a preference exists.
    fn has_preference(&self, name: &str) -> bool;

    /// Remove a preference.
    fn remove_preference(&mut self, name: &str);

    /// Save preferences to a file.
    fn save(&self, file_path: &str) -> Result<(), String>;

    /// Load preferences from a file.
    fn load(&mut self, file_path: &str) -> Result<(), String>;
}
