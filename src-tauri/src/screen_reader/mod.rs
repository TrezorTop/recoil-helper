/// Screen reader module for detecting images on the screen.
///
/// This module provides functionality to detect images on the screen using OpenCV template matching.
///
/// # Structure
///
/// The module is organized into several components:
/// - `reader`: Contains the main functions for screen reading operations
/// - `error`: Contains error types for screen reading operations
/// - `utils`: Utility functions for image processing and conversion
///
/// # Usage
///
/// The main entry point is the `detect_pattern` function, which can be used to
/// check if an image is present on the screen.
mod error;
pub(crate) mod reader;
pub(crate) mod utils;
