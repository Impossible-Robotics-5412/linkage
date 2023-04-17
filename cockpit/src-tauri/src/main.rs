// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod gamepad;

use commands::config::config;
use commands::gauge::start_gauge_connection;
use commands::linkage_lib::{disable, enable, LinkageLibState};

fn main() {
    logging::setup_logger(7642).expect("should be able to start logger");

    let gamepad_event_bus = gamepad::start_event_listener();

    tauri::Builder::default()
        .manage(LinkageLibState::new(gamepad_event_bus))
        .invoke_handler(tauri::generate_handler![
            enable,
            disable,
            config,
            start_gauge_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
