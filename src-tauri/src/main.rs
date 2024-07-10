// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::sync::{Arc, Mutex};

use crate::app_state::AppState;
use crate::json::Config;
use crate::mouse_controller::start_mouse_move_thread;

mod app_state;
mod json;
mod mouse_controller;

fn main() {
    let app_state = Arc::new(Mutex::new(AppState::new()));

    {
        let app_state_guard = app_state.lock().unwrap();
        start_mouse_move_thread(
            Arc::clone(&app_state_guard.closed),
            Arc::clone(&app_state_guard.pattern),
        )
    }

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![reload_config, save_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn reload_config(app_state: tauri::State<Arc<Mutex<AppState>>>) {
    let mut app_state_guard = app_state.lock().unwrap();
    app_state_guard.reload_config();
}

#[tauri::command]
fn save_config(app_state: tauri::State<Arc<Mutex<AppState>>>, config: Config) {
    json::write_config(config).unwrap();

    let mut app_state_guard = app_state.lock().unwrap();
    app_state_guard.reload_config();
}
