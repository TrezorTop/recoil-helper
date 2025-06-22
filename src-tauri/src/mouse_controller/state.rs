use crate::patterns::Steps;

/// Shared state for the mouse controller that can be updated at runtime
#[derive(Debug, Clone)]
pub struct MouseControllerState {
    /// The current pattern of mouse movements
    pub steps: Steps,
    /// Whether the controller is enabled
    pub enabled: bool,
}

impl MouseControllerState {
    /// Creates a new state with the given pattern
    pub fn with_pattern(pattern: Steps) -> Self {
        Self {
            steps: pattern,
            enabled: true,
        }
    }
}
