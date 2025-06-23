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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// A sequence of mouse movement steps that form a pattern
    pub config: Steps,
    /// Images associated with this pattern
    pub images: Vec<String>,
}

pub type Steps = Vec<Step>;
