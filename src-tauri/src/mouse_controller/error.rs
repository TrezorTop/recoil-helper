use std::fmt;

/// Error type for mouse input operations
#[derive(Debug)]
pub enum MouseInputError {
    /// Error when sending input to the system
    SendInputFailed,
}

impl fmt::Display for MouseInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MouseInputError::SendInputFailed => write!(f, "Failed to send mouse input"),
        }
    }
}

impl std::error::Error for MouseInputError {}