// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::thread;

use crate::app_state::AppState;

mod app_state;
mod json;
mod mouse_controller;
mod screen_reader;

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
        .invoke_handler(tauri::generate_handler![set_active_pattern, find_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Sets the active pattern in the application state.
///
/// This function takes the current application state, locks it, and sets the active pattern to the provided `pattern_name`.
/// The application state is managed using a shared `Arc<Mutex<AppState>>` instance, which is passed to this function.
/// This function is exposed as a Tauri command, allowing the frontend to interact with the backend to set the active pattern.
#[tauri::command]
fn set_active_pattern(app_state: tauri::State<Arc<Mutex<AppState>>>, pattern_name: String) {
    let mut app_state = app_state.lock().unwrap();
    app_state.set_active_pattern(&pattern_name);
}

#[tauri::command]
fn find_image() {
    thread::spawn(|| {
        screen_reader::contains_image();
    });
}
