use std::fmt;

/// Error type for screen reading operations
#[derive(Debug)]
pub enum ScreenReaderError {
    /// Error when capturing the screen
    ScreenCaptureFailed,
    /// Error when converting image formats
    ImageConversionFailed,
    /// Error when performing template matching
    TemplateMatchingFailed,
}

impl fmt::Display for ScreenReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScreenReaderError::ScreenCaptureFailed => write!(f, "Failed to capture screen"),
            ScreenReaderError::ImageConversionFailed => write!(f, "Failed to convert image format"),
            ScreenReaderError::TemplateMatchingFailed => write!(f, "Failed to perform template matching"),
        }
    }
}

impl std::error::Error for ScreenReaderError {}