// Re-export public items
mod controller;
mod error;
mod state;
mod step;
mod utils;

// Public exports
pub use controller::MouseController;
pub use step::{Pattern, Step};
