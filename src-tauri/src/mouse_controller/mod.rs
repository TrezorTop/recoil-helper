/// Mouse controller module for programmatic mouse movement control.
///
/// This module provides functionality to control mouse movements programmatically,
/// which can be used to implement recoil control or other mouse movement patterns.
///
/// # Structure
///
/// The module is organized into several components:
/// - `controller`: Contains the main `MouseController` that manages mouse movements
/// - `state`: Defines the state structure used by the controller
/// - `step`: Defines the structure for individual movement steps and patterns
/// - `error`: Contains error types for mouse input operations
/// - `utils`: Utility functions for mouse input and state detection
///
/// # Usage
///
/// The main entry point is the `MouseController` struct, which can be used to
/// create a controller and update its movement pattern.
mod controller;
mod error;
mod state;
mod step;
mod utils;

/// Public exports from the mouse_controller module
///
/// These are the primary types that users of this module will interact with:
/// - `MouseController`: The main controller for programmatic mouse movements
/// - `Pattern`: A sequence of mouse movement steps
/// - `Step`: A single mouse movement step with direction and duration
pub use controller::MouseController;
pub use step::{Pattern, Step};
