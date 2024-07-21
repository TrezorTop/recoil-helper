use std::collections::HashMap;
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::json;

/// The application state, containing the active pattern and the overall configuration.
pub struct AppState {
    pub active_pattern: Pattern,

    config: Config,
}

/// The configuration for the application, containing the available patterns.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub patterns: Patterns,
}

type Pattern = Vec<Step>;

type Patterns = HashMap<String, Pattern>;

/// A step in a pattern, containing the x and y offsets and the duration of the step.
///
/// This struct represents a single step in a pattern, which includes the x and y offsets
/// that define the movement of the step, as well as the duration of the step in milliseconds.
/// The `dx` and `dy` fields define the change in x and y coordinates, respectively, and the
/// `duration` field defines the time in milliseconds that the step should last.
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Step {
    pub dx: i32,
    pub dy: i32,
    pub duration: u64,
}

impl Default for AppState {
    /// Provides a default implementation for the `AppState` struct.
    ///
    /// This implementation reads the application configuration from a JSON file using the `json::read_config()`
    /// function, and sets the `active_pattern` field to the first pattern found in the configuration.
    /// If no patterns are found in the configuration, an error is returned.
    fn default() -> Self {
        let config = json::read_config().expect("Failed to read config");
        let active_pattern = config
            .patterns
            .values()
            .next()
            .expect("No patterns found in config")
            .clone();

        AppState {
            active_pattern,
            config,
        }
    }
}

impl AppState {
    /// Reads the application configuration from a JSON file and updates the `config` field of the `AppState` struct.
    ///
    /// This method reads the configuration from a JSON file using the `json::read_config()` function and assigns the
    /// resulting `Config` struct to the `config` field of the `AppState` struct. If the configuration file cannot be
    /// read or parsed, an error is returned.
    pub fn read_config(&mut self) -> Result<(), Box<dyn Error>> {
        let config = json::read_config()?;
        self.config = config;

        Ok(())
    }

    /// Sets the active pattern in the `AppState` to the pattern with the given name.
    ///
    /// This method looks up the pattern with the given name in the `config.patterns` map and
    /// updates the `active_pattern` field of the `AppState` struct with the found pattern.
    /// If no pattern is found with the given name, an error is returned.
    pub fn set_active_pattern(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        match self.config.patterns.get(name) {
            None => Err(format!("No pattern found with name: {}", name).into()),
            Some(value) => {
                self.active_pattern.clone_from(value);
                Ok(())
            }
        }
    }
}
