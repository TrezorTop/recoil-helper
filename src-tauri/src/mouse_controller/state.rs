use crate::mouse_controller::step::Pattern;

/// Shared state for the mouse controller that can be updated at runtime
#[derive(Debug, Clone)]
pub struct MouseControllerState {
    /// The current pattern of mouse movements
    pub pattern: Pattern,
    /// Whether the controller is enabled
    pub enabled: bool,
}

impl MouseControllerState {
    /// Creates a new state with the given pattern
    pub fn with_pattern(pattern: Pattern) -> Self {
        Self {
            pattern,
            enabled: true,
        }
    }
}
