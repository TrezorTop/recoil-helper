use crate::mouse_controller::MouseController;

use crate::patterns::PatternCollection;
use crate::screen_reader::ScreenReader;

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

            let pattern_collection = PatternCollection::new();
            let screen_reader = ScreenReader::new();

            // Create a mouse controller
            let mut mouse_controller = MouseController::new();

            // Detect which pattern is on the screen
            if let Some(pattern_name) =
                screen_reader.detect_pattern(pattern_collection.get_patterns())
            {
                println!("Detected pattern: {}", pattern_name);

                // Set the pattern for the controller
                mouse_controller.update_steps(pattern_collection.get_pattern(&pattern_name).config);
            } else {
                println!("No pattern detected");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
