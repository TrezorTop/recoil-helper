// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, thread};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::app_state::{AppState, PatternPart};

mod app_state;
mod json;
mod mouse_controller;

fn main() {
    let mut app_state = AppState::new();
    
    app_state.set_pattern(String::from("test"));

    start_mouse_move_thread(
        Arc::clone(&app_state.running),
        Arc::clone(&app_state.closed),
        Arc::clone(&app_state.pattern),
    );

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![start_mouse_move, stop_mouse_move])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn start_mouse_move_thread(
    running: Arc<Mutex<bool>>,
    closed: Arc<Mutex<bool>>,
    pattern: Arc<Mutex<Vec<PatternPart>>>,
) {
    thread::spawn(move || {
        let mut pattern_index = 0;
        let mut timer = Instant::now();

        loop {
            if *closed.lock().unwrap() {
                break;
            }

            let should_run = *running.lock().unwrap();

            if should_run {
                let pattern_guard = pattern.lock().unwrap();

                if pattern_index >= pattern_guard.len() {
                    pattern_index = 0;
                }

                let current_pattern_part = &pattern_guard[pattern_index];

                mouse_controller::send_mouse_input(current_pattern_part.x, current_pattern_part.y);

                if timer.elapsed() >= Duration::from_millis(current_pattern_part.delay)
                    && pattern_index < pattern_guard.len() - 1
                {
                    pattern_index = (pattern_index + 1) % pattern_guard.len();
                    timer = Instant::now();
                }
            } else {
                pattern_index = 0;
                timer = Instant::now();
            }

            thread::sleep(Duration::from_millis(16));
        }
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
