use crate::mouse_controller::MouseController;

use crate::patterns::PatternCollection;
use tauri::Manager;

mod mouse_controller;
mod patterns;

/// Default sleep duration between pattern processing iterations
pub const DEFAULT_THREAD_SLEEP_DURATION_MS: u64 = 16;

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

            // Create a mouse controller
            let mut mouse_controller = MouseController::create();

            // Set the pattern for the controller (this will automatically start the controller)
            mouse_controller.update_steps(pattern_collection.get_pattern("r4c").config);

            // Store the controller handle in the app state for later use
            // This allows other parts of the application to update the pattern at runtime
            app.manage(mouse_controller);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
