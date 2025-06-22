use crate::mouse_controller::{MouseController, Pattern, Step};

use tauri::Manager;

mod mouse_controller;

/// Default sleep duration between pattern processing iterations
pub const DEFAULT_THREAD_SLEEP_DURATION_MS: u64 = 8;

/// Default recoil compensation pattern
fn default_pattern() -> Pattern {
    vec![
        Step {
            dx: 0,
            dy: 5,
            duration: 250,
        },
        Step {
            dx: 0,
            dy: 10,
            duration: 250,
        },
        Step {
            dx: 0,
            dy: 25,
            duration: 250,
        },
    ]
}

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

            // Create a mouse controller
            let mut mouse_controller = MouseController::create();

            // Set the pattern for the controller (this will automatically start the controller)
            mouse_controller.update_pattern(default_pattern());

            // Store the controller handle in the app state for later use
            // This allows other parts of the application to update the pattern at runtime
            app.manage(mouse_controller);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
