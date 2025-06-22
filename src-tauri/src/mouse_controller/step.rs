/// Represents a single step in a mouse movement pattern.
#[derive(Debug, Clone)]
pub struct Step {
    /// Duration to wait after this step in milliseconds
    pub duration: u64,
    /// Horizontal movement in pixels
    pub dx: i32,
    /// Vertical movement in pixels
    pub dy: i32,
}

/// A sequence of mouse movement steps that form a pattern
pub type Pattern = Vec<Step>;
