// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tauri::Manager;

use crate::app_state::AppState;

mod app_state;
mod mouse_controller;

fn main() {
    let app_state = AppState::new();

    start_mouse_move_thread(
        Arc::clone(&app_state.running),
        Arc::clone(&app_state.closed),
    );

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![start_mouse_move, stop_mouse_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn start_mouse_move_thread(running: Arc<Mutex<bool>>, closed: Arc<Mutex<bool>>) {
    thread::spawn(move || loop {
        // creating a new scope to release the lock fast as possible
        let should_run = {
            let is_running = running.lock().unwrap();
            *is_running
        };

        let app_closed = {
            let is_closed = closed.lock().unwrap();
            *is_closed
        };

        if should_run {
            mouse_controller::send_mouse_input(0, 10);
        }

        if app_closed {
            break;
        }

        thread::sleep(Duration::from_millis(16));
    });
}

#[tauri::command]
fn start_mouse_move(app_state: tauri::State<AppState>) {
    let mut is_running = app_state.running.lock().unwrap();
    *is_running = true;
}

#[tauri::command]
fn stop_mouse_move(app_state: tauri::State<AppState>) {
    let mut is_running = app_state.running.lock().unwrap();
    *is_running = false;
}
