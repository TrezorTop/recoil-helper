use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use winapi::um::winuser::GetAsyncKeyState;

/// Constant for the high-order bit that indicates if a key is pressed
const KEY_PRESSED_MASK: i16 = 0x8000u16 as i16;

/// Type alias for key press callback functions
pub type KeyPressCallback = Box<dyn Fn() + Send + 'static>;

/// Represents a key on the keyboard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    /// Virtual key code
    pub code: i32,
    /// Human-readable representation of the key
    pub name: &'static str,
}

/// Common keyboard keys
pub mod keys {
    use super::Key;

    pub const KEY_1: Key = Key {
        code: 0x31,
        name: "1",
    };
    pub const KEY_2: Key = Key {
        code: 0x32,
        name: "2",
    };
}

/// Keyboard listener that monitors key presses and executes callbacks
pub struct KeyboardListener {
    /// Map of keys to their callbacks
    callbacks: Arc<Mutex<HashMap<Key, KeyPressCallback>>>,
    /// Map of keys to their last known state
    key_states: Arc<Mutex<HashMap<Key, bool>>>,
    /// Poll interval in milliseconds
    poll_interval: u64,
    /// Flag to control the listening loop
    running: Arc<Mutex<bool>>,
}

impl KeyboardListener {
    /// Creates a new keyboard listener with the default poll interval
    pub fn new() -> Self {
        Self::with_poll_interval(50)
    }

    /// Creates a new keyboard listener with a custom poll interval
    pub fn with_poll_interval(poll_interval: u64) -> Self {
        KeyboardListener {
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            key_states: Arc::new(Mutex::new(HashMap::new())),
            poll_interval,
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Registers a callback function for a specific key
    pub fn on_key_press<F>(&mut self, key: Key, callback: F)
    where
        F: Fn() + Send + 'static,
    {
        if let Ok(mut callbacks) = self.callbacks.lock() {
            callbacks.insert(key, Box::new(callback));
        }

        // Initialize key state if not already present
        if let Ok(mut states) = self.key_states.lock() {
            states.entry(key).or_insert(false);
        }
    }

    /// Checks if a specific key is currently pressed
    fn is_key_pressed(key_code: i32) -> bool {
        unsafe { GetAsyncKeyState(key_code) & KEY_PRESSED_MASK != 0 }
    }

    /// Starts the keyboard listening loop in a background thread
    pub fn start(&self) -> thread::JoinHandle<()> {
        // Set running flag to true
        if let Ok(mut running) = self.running.lock() {
            *running = true;
        }

        // Clone Arc references for the background thread
        let callbacks = self.callbacks.clone();
        let key_states = self.key_states.clone();
        let running = self.running.clone();
        let poll_interval = self.poll_interval;

        // Start the background thread
        thread::spawn(move || {
            while let Ok(is_running) = running.lock() {
                if !*is_running {
                    break;
                }

                // Check each registered key
                if let Ok(mut states) = key_states.lock() {
                    for (key, last_pressed) in states.iter_mut() {
                        let currently_pressed = Self::is_key_pressed(key.code);

                        // Only trigger on key press (not on hold)
                        if currently_pressed && !*last_pressed {
                            if let Ok(callbacks) = callbacks.lock() {
                                if let Some(callback) = callbacks.get(key) {
                                    callback();
                                }
                            }
                        }

                        // Update the key state
                        *last_pressed = currently_pressed;
                    }
                }

                // Sleep to avoid high CPU usage
                thread::sleep(Duration::from_millis(poll_interval));
            }
        })
    }

    /// Stops the keyboard listening loop
    pub fn stop(&self) {
        if let Ok(mut running) = self.running.lock() {
            *running = false;
        }
    }
}

// For backward compatibility
/// Checks if the "1" key is currently pressed
pub fn is_one_key_pressed() -> bool {
    KeyboardListener::is_key_pressed(keys::KEY_1.code)
}
