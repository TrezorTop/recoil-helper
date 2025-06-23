use crate::mouse_controller::MouseController;
use crate::patterns::PatternCollection;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

            // Detect and set pattern once initially
            if let Ok(mut controller) = mouse_controller.lock() {
                detect_and_set_pattern(&pattern_collection, &mut controller);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
