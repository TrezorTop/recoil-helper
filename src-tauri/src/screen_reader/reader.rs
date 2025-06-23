use opencv::core::{min_max_loc, Mat, Point};
use opencv::{imgcodecs, imgproc};
use screenshots::Screen;
use std::collections::HashMap;

use crate::patterns::Pattern;
use crate::screen_reader::utils::rgba_image_to_mat;

/// The threshold value used to determine if the screen contains the target image.
/// A value greater than this threshold indicates the target image was found on the screen.
const IMAGE_THRESHOLD: f64 = 0.9;

/// Main struct for screen reading operations
#[derive(Debug)]
pub struct ScreenReader;

impl ScreenReader {
    /// Creates a new ScreenReader instance
    pub fn new() -> Self {
        Self
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
    pub fn detect_pattern(&self, patterns: &HashMap<String, Pattern>) -> Option<String> {
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
        let screen_mat = match rgba_image_to_mat(&image_screen) {
            Ok(mat) => mat,
            Err(_) => return None,
        };

        // Check each pattern's images against the captured screen
        for (pattern_name, pattern) in patterns {
            // All images in the pattern must be found
            let mut all_images_found = true;

            for image_path in &pattern.images {
                let full_path = format!("config/images/{}", image_path);
                
                if !self.match_template(&screen_mat, full_path) {
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

    /// Matches a template image against a screen matrix
    ///
    /// # Arguments
    /// * `screen_mat` - The screen matrix to search in
    /// * `image_path` - Path to the template image
    ///
    /// # Returns
    /// * `bool` - True if the template was found on the screen, false otherwise
    fn match_template(&self, screen_mat: &Mat, image_path: String) -> bool {
        // Read the template image
        let template_mat = match imgcodecs::imread(image_path.as_ref(), imgcodecs::IMREAD_UNCHANGED) {
            Ok(mat) => mat,
            Err(_) => return false, // Skip this image if it can't be read
        };

        let mut result = Mat::default();

        // Perform template matching
        if imgproc::match_template(
            screen_mat,
            &template_mat,
            &mut result,
            imgproc::TM_CCOEFF_NORMED,
            &Mat::default(),
        )
        .is_err()
        {
            return false; // Skip this image if template matching fails
        }

        // Find the minimum and maximum values in the result
        let mut min_val = 0.0;
        let mut max_val = 0.0;
        let mut min_loc = Point::default();
        let mut max_loc = Point::default();

        if min_max_loc(
            &result,
            Some(&mut min_val),
            Some(&mut max_val),
            Some(&mut min_loc),
            Some(&mut max_loc),
            &Mat::default(),
        )
        .is_err()
        {
            return false; // Skip this image if min_max_loc fails
        }

        // If the image is found on the screen, return true
        max_val > IMAGE_THRESHOLD
    }
}
