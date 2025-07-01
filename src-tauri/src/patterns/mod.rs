mod collection;
mod types;

pub use collection::PatternCollection;
// Re-export the types for use by other modules
pub use types::{Pattern, Sensitivity, Step, Steps};
