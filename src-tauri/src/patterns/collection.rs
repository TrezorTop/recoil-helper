use crate::patterns::types::Pattern;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

/// Collection of named patterns that can be loaded from and saved to JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternCollection {
    /// Map of pattern names to patterns
    patterns: HashMap<String, Pattern>,
}

impl PatternCollection {
    /// Creates a new empty pattern collection
    pub fn new() -> Self {
        let patterns = Self::load_from_file();

        Self {
            patterns: patterns
                .expect("Failed to load patterns from file")
                .patterns,
        }
    }

    /// Gets a pattern by name
    pub fn get_pattern(&self, name: &str) -> Pattern {
        self.patterns.get(name).expect("Pattern not found").clone()
    }

    /// Loads patterns from a JSON file
    fn load_from_file() -> Result<Self, String> {
        match fs::read_to_string("config/config.json") {
            Ok(json) => match serde_json::from_str(&json) {
                Ok(collection) => Ok(collection),
                Err(e) => Err(format!("Failed to parse JSON: {}", e)),
            },
            Err(e) => Err(format!("Failed to read file: {}", e)),
        }
    }
}
