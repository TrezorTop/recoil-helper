use serde::{Deserialize, Serialize};

/// Represents a single step in a mouse movement pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    /// Duration to wait after this step in milliseconds
    pub duration: u64,
    /// Horizontal movement in pixels
    pub dx: i32,
    /// Vertical movement in pixels
    pub dy: i32,
    /// Pre-calculated adjusted horizontal movement based on sensitivity
    #[serde(skip, default = "default_to_dx")]
    pub adjusted_dx: i32,
    /// Pre-calculated adjusted vertical movement based on sensitivity
    #[serde(skip, default = "default_to_dy")]
    pub adjusted_dy: i32,
}

// Default function for adjusted_dx that returns the value of dx
fn default_to_dx() -> i32 {
    0 // This will be overwritten during update_steps
}

// Default function for adjusted_dy that returns the value of dy
fn default_to_dy() -> i32 {
    0 // This will be overwritten during update_steps
}

/// Represents sensitivity settings for mouse movements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensitivity {
    /// Horizontal sensitivity (1.0 is lowest, higher values reduce movement)
    pub x: f32,
    /// Vertical sensitivity (1.0 is lowest, higher values reduce movement)
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// A sequence of mouse movement steps that form a pattern
    pub config: Steps,
    /// Images associated with this pattern
    pub images: Vec<String>,
}

pub type Steps = Vec<Step>;
