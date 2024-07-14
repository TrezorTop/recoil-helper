use std::collections::HashMap;

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
    /// Creates a new `AppState` instance with the default configuration.
    ///
    /// This method reads the application configuration from a JSON file and initializes the `AppState`
    /// struct with the default active pattern and the loaded configuration. The active pattern is
    /// set to the first pattern found in the `config.patterns` map.
    fn default() -> Self {
        let config = json::read_config().unwrap();

        let active_pattern = config.patterns.values().next().unwrap().clone();

        AppState {
            active_pattern,
            config,
        }
    }
}

impl AppState {
    /// Reads the application configuration from a JSON file and updates the `config` field of the `AppState` struct.
    ///
    /// This method is responsible for loading the application configuration from a JSON file and updating the `config` field
    /// of the `AppState` struct with the loaded configuration. The configuration is expected to be in a valid JSON format
    /// and contain the necessary information for the application, such as the available patterns.
    pub fn read_config(&mut self) {
        let config = json::read_config().unwrap();
        self.config = config;
    }

    /// Sets the active pattern in the `AppState` to the pattern with the given name.
    ///
    /// If a pattern with the given name is found in the `config.patterns` map, it is cloned and
    /// assigned to the `active_pattern` field of the `AppState`. If no pattern is found with the
    /// given name, a warning is printed to the console.
    pub fn set_active_pattern(&mut self, name: &str) {
        let pattern = self.config.patterns.get(name);

        match pattern {
            None => {
                eprintln!("No pattern found with name: {}", name);
            }
            Some(value) => {
                self.active_pattern.clone_from(value);
            }
        }
    }
}
