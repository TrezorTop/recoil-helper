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
    patterns: Patterns,
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
    fn default() -> Self {
        let config = json::read_config().unwrap();

        let first_pattern = config.patterns.values().next().unwrap();
        let active_pattern = (*first_pattern).clone();

        AppState {
            active_pattern,
            config,
        }
    }
}
