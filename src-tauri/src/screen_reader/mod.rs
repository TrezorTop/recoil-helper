/// Screen reader module for detecting images on the screen.
///
/// This module provides functionality to detect images on the screen using OpenCV template matching.
///
/// # Structure
///
/// The module is organized into several components:
/// - `reader`: Contains the main `ScreenReader` that manages screen reading operations
/// - `error`: Contains error types for screen reading operations
/// - `utils`: Utility functions for image processing and conversion
///
/// # Usage
///
/// The main entry point is the `ScreenReader` struct, which can be used to
/// create a reader and check if an image is present on the screen.
mod error;
mod reader;
mod utils;

/// Public exports from the screen_reader module
///
/// These are the primary types that users of this module will interact with:
/// - `ScreenReader`: The main controller for screen reading operations
pub use reader::ScreenReader;
