// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gamepad;
mod linkage_lib;

use linkage_lib::{disable, enable, LinkageLibState};

fn main() {
    common::logging::setup_logger(7642).expect("should be able to start logger");

    let gamepad_event_bus = gamepad::start_event_listener();

    tauri::Builder::default()
        .manage(LinkageLibState::new(gamepad_event_bus))
        .invoke_handler(tauri::generate_handler![enable, disable])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
