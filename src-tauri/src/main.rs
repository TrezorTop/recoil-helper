// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread;

mod mouse_controller;

fn main() {
    start_main_loop();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn start_main_loop() {
    thread::spawn(|| loop {
        mouse_controller::send_mouse_input(1, 1);
        thread::sleep(std::time::Duration::from_millis(1));
    });
}
