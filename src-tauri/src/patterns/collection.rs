use crate::patterns::types::Pattern;
use screenshots::Screen;
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

    /// Gets a reference to the patterns map
    pub fn get_patterns(&self) -> &HashMap<String, Pattern> {
        &self.patterns
    }

    /// Detects which pattern is currently on the screen
    ///
    /// This function checks all images from all patterns and returns the name
    /// of the first pattern whose image was detected on the screen.
    ///
    /// This is an optimized version that captures the screen only once for all pattern checks,
    /// rather than capturing it for each image check.
    ///
    /// # Arguments
    /// * `patterns` - A map of pattern names to patterns
    ///
    /// # Returns
    /// * `Option<String>` - The name of the detected pattern, or None if no pattern was detected
    pub fn detect_pattern(&self) -> Option<String> {
        let patterns = &self.patterns;

        // Capture the primary screen once
        let screen = match Screen::all() {
            Ok(s) => s,
            Err(_) => return None,
        };

        let primary_screen = screen.first()?;

        let image_screen = match primary_screen.capture() {
            Ok(img) => img,
            Err(_) => return None,
        };

        // Convert the captured screen to an OpenCV Mat once
        let screen_mat = match crate::screen_reader::utils::rgba_image_to_mat(&image_screen) {
            Ok(mat) => mat,
            Err(_) => return None,
        };

        // Check each pattern's images against the captured screen
        for (pattern_name, pattern) in patterns {
            // All images in the pattern must be found
            let mut all_images_found = true;

            for image_path in &pattern.images {
                let full_path = format!("config/images/{}", image_path);

                if !crate::screen_reader::reader::match_template(&screen_mat, full_path) {
                    all_images_found = false;
                    break;
                }
            }

            // If all images were found, return this pattern
            if all_images_found && !pattern.images.is_empty() {
                return Some(pattern_name.clone());
            }
        }
        None
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
