use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use log::{debug, error, info};

use crate::mouse_controller::state::MouseControllerState;
use crate::mouse_controller::step::Step;
use crate::mouse_controller::utils::{send_mouse_input, should_run};
use crate::DEFAULT_THREAD_SLEEP_DURATION_MS;

/// Controller for programmatic mouse movements
pub struct MouseController {
    state: Arc<std::sync::RwLock<MouseControllerState>>,
}

impl MouseController {
    /// Creates a new MouseController and automatically starts the internal thread
    ///
    /// # Returns
    /// A MouseController that can be used to update the pattern
    pub fn create() -> Self {
        // Create a shared state with an empty pattern
        let state = Arc::new(std::sync::RwLock::new(MouseControllerState::with_pattern(
            vec![],
        )));

        // Start the controller thread
        Self::start_controller_thread(Arc::clone(&state));

        // Return a new MouseController with the state and thread
        Self {
            state,
        }
    }

    /// Updates the pattern used by the mouse controller
    pub fn update_pattern(&mut self, pattern: crate::mouse_controller::step::Pattern) {
        if let Ok(mut state) = self.state.write() {
            state.pattern = pattern;
            info!(
                "Updated mouse controller pattern with {} steps",
                state.pattern.len()
            );
        } else {
            error!("Failed to acquire write lock on mouse controller state");
        }
    }

    // Private method to start the controller thread
    fn start_controller_thread(
        state: Arc<std::sync::RwLock<MouseControllerState>>,
    ) -> thread::JoinHandle<()> {
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
                            let is_empty = guard.pattern.is_empty();

                            // If pattern is empty, don't clone anything
                            if is_empty {
                                (vec![], guard.enabled, true)
                            } else {
                                // Only clone the pattern, not the entire state
                                (guard.pattern.clone(), guard.enabled, false)
                            }
                        },
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
                        // Send mouse input based on the current step
                        if let Err(e) = send_mouse_input(step.dx, step.dy) {
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
        })
    }
}
