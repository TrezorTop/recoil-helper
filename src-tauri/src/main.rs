// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use crate::app_state::AppState;

mod app_state;
mod json;
mod mouse_controller;

/// The main entry point for the application.
///
/// This function sets up the application state, starts the mouse controller, and runs the Tauri application.
/// The application state is managed using a shared `Arc<Mutex<AppState>>` instance, which is passed to the mouse controller and the Tauri application.
/// The Tauri application is configured to use the generated invoke handlers, which allow the frontend to interact with the backend.
fn main() {
    // Create a shared app state instance.
    let app_state = Arc::new(Mutex::new(AppState::default()));

    // Start the mouse controller.
    mouse_controller::start_mouse_controller(Arc::clone(&app_state));

    // Run the Tauri application.
    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
