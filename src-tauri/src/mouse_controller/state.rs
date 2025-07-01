use crate::patterns::{Sensitivity, Steps};

/// Shared state for the mouse controller that can be updated at runtime
#[derive(Debug, Clone)]
pub struct MouseControllerState {
    /// The current pattern of mouse movements
    pub steps: Option<Steps>,
    /// Whether the controller is enabled
    pub enabled: bool,
    /// Sensitivity settings for mouse movements
    pub sensitivity: Sensitivity,
}

impl MouseControllerState {
    /// Creates a new state with the given pattern
    pub fn with_pattern(pattern: Steps) -> Self {
        Self {
            steps: Some(pattern),
            enabled: true,
            sensitivity: Sensitivity { x: 1.0, y: 1.0 },
        }
    }

    /// Creates a new state with the given pattern and sensitivity
    pub fn with_pattern_and_sensitivity(pattern: Steps, sensitivity: Sensitivity) -> Self {
        Self {
            steps: Some(pattern),
            enabled: true,
            sensitivity,
        }
    }
}
