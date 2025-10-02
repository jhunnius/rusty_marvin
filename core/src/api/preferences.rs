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
