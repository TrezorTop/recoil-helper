use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use log::{debug, error, info};

use crate::mouse_controller::state::MouseControllerState;
use crate::mouse_controller::utils::{send_mouse_input, should_run};
use crate::patterns::{PatternCollection, Sensitivity, Step, Steps};

/// Default sleep duration between pattern processing iterations
const DEFAULT_THREAD_SLEEP_DURATION_MS: u64 = 24;

/// Controller for programmatic mouse movements
///
/// This struct is the main entry point for the mouse controller functionality.
/// It manages a background thread that applies mouse movements according to a
/// configured pattern when specific conditions are met (e.g., when certain
/// mouse buttons are pressed).
pub struct MouseController {
    /// Shared state that can be accessed and modified by both the controller
    /// and the background thread. Protected by a read-write lock to ensure
    /// thread safety.
    state: Arc<std::sync::RwLock<MouseControllerState>>,
}

impl MouseController {
    /// Creates a new MouseController and automatically starts the internal thread
    ///
    /// This is the main constructor for the MouseController. It initializes the
    /// controller with an empty pattern and starts a background thread that will
    /// monitor mouse button states and apply the pattern when conditions are met.
    ///
    /// # Thread Management
    /// This method spawns a background thread that runs for the lifetime of the
    /// program. The thread is not joined or stopped explicitly, as it's expected
    /// to run until program termination.
    ///
    /// # Returns
    /// A MouseController instance that can be used to update the movement pattern
    pub fn new() -> Self {
        // Load sensitivity settings from configuration
        let pattern_collection = PatternCollection::new();
        let sensitivity = pattern_collection.sensitivity.clone();

        // Create a shared state with an empty pattern and sensitivity settings
        let state = Arc::new(std::sync::RwLock::new(
            MouseControllerState::with_pattern_and_sensitivity(vec![], sensitivity),
        ));

        // Start the controller thread
        Self::start_controller_thread(Arc::clone(&state));

        // Return a new MouseController with the state and thread
        Self { state }
    }

    /// Updates the pattern used by the mouse controller
    ///
    /// This method allows changing the mouse movement pattern at runtime.
    /// The new pattern will be used for all later mouse movements.
    ///
    /// # Arguments
    /// * `pattern` - A vector of Step objects that define the mouse movement pattern.
    ///   Each step specifies horizontal and vertical movement amounts and a duration.
    ///
    /// # Thread Safety
    /// This method acquires a write lock on the shared state, ensuring that
    /// the pattern is not being read by the background thread while it's being updated.
    pub fn update_steps(&mut self, mut steps: Steps) {
        if let Ok(mut state) = self.state.write() {
            // Pre-calculate adjusted dx and dy values based on sensitivity
            for step in &mut steps {
                // Apply sensitivity to dx and dy values
                // Higher sensitivity values make the movement smaller (divided by sensitivity)
                step.adjusted_dx = if step.dx == 0 {
                    0
                } else if state.sensitivity.x > 1.0 {
                    let calculated = (step.dx as f32 / state.sensitivity.x) as i32;
                    if calculated == 0 {
                        if step.dx > 0 { 1 } else { -1 }
                    } else {
                        calculated
                    }
                } else {
                    step.dx
                };

                step.adjusted_dy = if step.dy == 0 {
                    0
                } else if state.sensitivity.y > 1.0 {
                    let calculated = (step.dy as f32 / state.sensitivity.y) as i32;
                    if calculated == 0 {
                        if step.dy > 0 { 1 } else { -1 }
                    } else {
                        calculated
                    }
                } else {
                    step.dy
                };
            }

            state.steps = Some(steps);
        } else {
            error!("Failed to acquire write lock on mouse controller state");
        }
    }

    /// Sets the pattern to None (null)
    ///
    /// This method is used when no pattern is detected and we want to disable
    /// the mouse controller by setting the pattern to None.
    ///
    /// # Thread Safety
    /// This method acquires a write lock on the shared state, ensuring that
    /// the pattern is not being read by the background thread while it's being updated.
    pub fn clear_steps(&mut self) {
        if let Ok(mut state) = self.state.write() {
            state.steps = None;
            info!("Cleared mouse controller pattern (set to None)");
        } else {
            error!("Failed to acquire write lock on mouse controller state");
        }
    }

    /// Updates the sensitivity settings for mouse movements
    ///
    /// # Arguments
    /// * `sensitivity` - A Sensitivity object that defines the horizontal and vertical sensitivity.
    ///   Higher values result in smaller mouse movements.
    ///
    /// # Thread Safety
    /// This method acquires a write lock on the shared state, ensuring that
    /// the sensitivity is not being read by the background thread while it's being updated.
    pub fn update_sensitivity(&mut self, sensitivity: Sensitivity) {
        if let Ok(mut state) = self.state.write() {
            // Store the new sensitivity values
            let sensitivity_x = sensitivity.x;
            let sensitivity_y = sensitivity.y;

            // Update the sensitivity in the state
            state.sensitivity = sensitivity;

            // Recalculate adjusted values for all steps if steps exist
            if let Some(steps) = &mut state.steps {
                for step in steps {
                    // Apply sensitivity to dx and dy values
                    // Higher sensitivity values make the movement smaller (divided by sensitivity)
                    step.adjusted_dx = if step.dx == 0 {
                        0
                    } else if sensitivity_x > 1.0 {
                        let calculated = (step.dx as f32 / sensitivity_x) as i32;
                        if calculated == 0 {
                            if step.dx > 0 { 1 } else { -1 }
                        } else {
                            calculated
                        }
                    } else {
                        step.dx
                    };

                    step.adjusted_dy = if step.dy == 0 {
                        0
                    } else if sensitivity_y > 1.0 {
                        let calculated = (step.dy as f32 / sensitivity_y) as i32;
                        if calculated == 0 {
                            if step.dy > 0 { 1 } else { -1 }
                        } else {
                            calculated
                        }
                    } else {
                        step.dy
                    };
                }
            }

            info!(
                "Updated mouse controller sensitivity to x={}, y={} and recalculated adjusted values",
                sensitivity_x, sensitivity_y
            );
        } else {
            error!("Failed to acquire write lock on mouse controller state");
        }
    }

    /// Starts a background thread that continuously monitors the state and applies
    /// mouse movements according to the configured pattern when conditions are met.
    ///
    /// This method is called automatically when a new MouseController is created.
    /// The thread runs indefinitely until the program terminates.
    ///
    /// # Implementation Details
    /// The thread performs the following operations in a loop:
    /// 1. Reads the current pattern and enabled state from the shared state
    /// 2. Checks if the pattern is empty and skips the iteration if it is
    /// 3. Determines if the controller should be running based on mouse button states
    /// 4. Applies mouse movements according to the current step in the pattern
    /// 5. Advances to the next step when the current step's duration has elapsed
    ///
    /// # Performance Considerations
    /// - The thread minimizes lock duration by only acquiring the read lock briefly
    /// - It uses caching to avoid repeated pattern access and cloning
    /// - State changes are logged only when they occur to reduce logging overhead
    ///
    /// # Arguments
    /// * `state` - Shared state that can be accessed by both the controller and the thread
    fn start_controller_thread(state: Arc<std::sync::RwLock<MouseControllerState>>) {
        info!("Starting mouse controller thread");

        thread::spawn(move || {
            // Variables to track the pattern progress
            let mut current_step_index = 0;
            let mut last_step_time = Instant::now();
            let mut is_running = false;

            // Cache for the current step to avoid repeated array access
            let mut current_step: Option<Step> = None;

            loop {
                // Get only what we need from the state, minimizing lock duration
                let (pattern, enabled, pattern_is_empty) = {
                    match state.read() {
                        Ok(guard) => {
                            // Check if steps is None or empty
                            let is_empty = guard.steps.as_ref().map_or(true, |s| s.is_empty());

                            // If pattern is None or empty, don't clone anything
                            if is_empty {
                                (vec![], guard.enabled, true)
                            } else {
                                // Only clone the pattern, not the entire state
                                (guard.steps.as_ref().unwrap().clone(), guard.enabled, false)
                            }
                        }
                        Err(e) => {
                            error!("Failed to acquire read lock on state: {}", e);
                            thread::sleep(Duration::from_millis(DEFAULT_THREAD_SLEEP_DURATION_MS));
                            continue;
                        }
                    }
                }; // Lock is released here

                // Check if the pattern is empty
                if pattern_is_empty {
                    debug!("Pattern is empty, skipping iteration");
                    thread::sleep(Duration::from_millis(DEFAULT_THREAD_SLEEP_DURATION_MS));
                    continue;
                }

                // Check if the controller is enabled and should be running
                let should_be_running = enabled && should_run();

                // Only log state changes to reduce logging overhead
                if should_be_running != is_running {
                    if should_be_running {
                        debug!("Mouse controller activated");
                        // Always reset to step 1 when the controller is activated
                        current_step_index = 0;
                        last_step_time = Instant::now();
                        // Update the cached step
                        current_step = Some(pattern[0].clone());
                        debug!("Mouse buttons pressed, resetting to step 1");
                    } else {
                        debug!("Mouse controller deactivated");
                    }
                    is_running = should_be_running;
                }

                if is_running {
                    // Check if the current step index is out of bounds of the pattern
                    // This can happen if the pattern was updated while the controller was running
                    if current_step_index >= pattern.len() {
                        debug!("Current step index out of bounds, resetting to step 0");
                        current_step_index = 0;
                        last_step_time = Instant::now();
                        // Update the cached step
                        current_step = Some(pattern[0].clone());
                    }

                    // Use the cached step if available, otherwise update it
                    if current_step.is_none() {
                        current_step = Some(pattern[current_step_index].clone());
                    }

                    // Get the current step (safe because we've checked bounds and emptiness)
                    if let Some(step) = &current_step {
                        // Use pre-calculated adjusted values
                        // Send mouse input based on the adjusted values

                        if let Err(e) = send_mouse_input(step.adjusted_dx, step.adjusted_dy) {
                            error!("Failed to send mouse input: {}", e);
                        }

                        // Check if it's time to move to the next step
                        if last_step_time.elapsed() >= Duration::from_millis(step.duration) {
                            // If we're not at the last step, move to the next step
                            if current_step_index < pattern.len() - 1 {
                                current_step_index += 1;
                                // Update the cached step
                                current_step = Some(pattern[current_step_index].clone());
                            }
                            // Otherwise, stay at the last step (don't loop back to the beginning)
                            last_step_time = Instant::now();
                        }
                    }
                }

                thread::sleep(Duration::from_millis(DEFAULT_THREAD_SLEEP_DURATION_MS));
            }
        });
    }
}
