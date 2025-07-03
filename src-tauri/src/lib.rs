use crate::keyboard_listener::{keys, KeyboardListener};
use crate::mouse_controller::MouseController;
use crate::patterns::PatternCollection;
use log::{error, info};
use std::sync::{Arc, Mutex};
use tauri::Emitter;

mod keyboard_listener;
mod mouse_controller;
mod patterns;
mod screen_reader;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Create instances of services
            let pattern_collection = Arc::new(PatternCollection::new());
            let mouse_controller = Arc::new(Mutex::new(MouseController::new()));

            // Create a keyboard listener
            let mut keyboard_listener = KeyboardListener::new();

            // Get the app handle for emitting events to the frontend
            let app_handle = app.handle();

            // Register callbacks for keys
            register_key_callback(
                app_handle.clone(),
                &mut keyboard_listener,
                keys::KEY_1,
                &pattern_collection,
                &mouse_controller,
            );
            register_key_callback(
                app_handle.clone(),
                &mut keyboard_listener,
                keys::KEY_2,
                &pattern_collection,
                &mouse_controller,
            );
            register_reload_callback(
                app_handle.clone(),
                &mut keyboard_listener,
                keys::KEY_F1,
                &pattern_collection,
                &mouse_controller,
            );

            // Start the keyboard listener
            keyboard_listener.start();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Registers a callback for a specific key that will detect and set patterns
fn register_key_callback(
    app_handle: tauri::AppHandle,
    keyboard_listener: &mut KeyboardListener,
    key: keyboard_listener::Key,
    pattern_collection: &Arc<PatternCollection>,
    mouse_controller: &Arc<Mutex<MouseController>>,
) {
    let pattern_collection_clone = pattern_collection.clone();
    let mouse_controller_clone = mouse_controller.clone();
    let app_handle_clone = app_handle.clone();
    keyboard_listener.on_key_press(key, move || {
        if let Ok(mut controller) = mouse_controller_clone.lock() {
            if let Some(pattern_name) =
                detect_and_set_pattern(&pattern_collection_clone, &mut controller)
            {
                // Emit the pattern name to the frontend
                let _ = app_handle_clone.emit("pattern-selected", pattern_name);
            } else {
                // Emit null pattern to the frontend
                let _ = app_handle_clone.emit("pattern-selected", "");
            }
        }
    });
}

fn detect_and_set_pattern(
    pattern_collection: &PatternCollection,
    mouse_controller: &mut MouseController,
) -> Option<String> {
    // Detect which pattern is on the screen
    if let Some(pattern_name) = pattern_collection.detect_pattern() {
        println!("Detected pattern: {}", pattern_name);

        // Set the pattern for the controller
        mouse_controller.update_steps(pattern_collection.get_pattern(&pattern_name).config);

        Some(pattern_name)
    } else {
        println!("No pattern detected");

        // Set null (None) to mouse_controller when no pattern is detected
        mouse_controller.clear_steps();

        None
    }
}

/// Registers a callback for the F1 key that will reload the JSON config
fn register_reload_callback(
    app_handle: tauri::AppHandle,
    keyboard_listener: &mut KeyboardListener,
    key: keyboard_listener::Key,
    pattern_collection: &Arc<PatternCollection>,
    mouse_controller: &Arc<Mutex<MouseController>>,
) {
    let pattern_collection_clone = Arc::clone(pattern_collection);
    let mouse_controller_clone = Arc::clone(mouse_controller);
    let app_handle_clone = app_handle.clone();

    keyboard_listener.on_key_press(key, move || {
        info!("Reloading JSON config...");

        // Create a mutable reference to the pattern collection
        let pattern_collection_mut =
            unsafe { &mut *(Arc::as_ptr(&pattern_collection_clone) as *mut PatternCollection) };

        // Reload the JSON config
        match pattern_collection_mut.reload() {
            Ok(()) => {
                info!("JSON config reloaded successfully");

                // Update the sensitivity settings
                if let Ok(mut controller) = mouse_controller_clone.lock() {
                    // Update sensitivity from the reloaded config
                    controller.update_sensitivity(pattern_collection_clone.sensitivity.clone());

                    // Update the active pattern
                    if let Some(pattern_name) =
                        detect_and_set_pattern(&pattern_collection_clone, &mut controller)
                    {
                        // Emit the pattern name to the frontend
                        let _ = app_handle_clone.emit("pattern-selected", pattern_name);
                    } else {
                        // Emit null pattern to the frontend
                        let _ = app_handle_clone.emit("pattern-selected", "");
                    }
                }
            }
            Err(e) => {
                error!("Failed to reload JSON config: {}", e);
            }
        }
    });
}
