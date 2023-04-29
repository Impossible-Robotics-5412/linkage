// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use crate::commands::config::ConfigState;
use crate::commands::gamepad::GamepadState;
use commands::config::get_config;
use commands::config::set_cockpit_config;
use commands::gamepad::start_gamepad_event_listener;
use commands::gauge::start_gauge_connection;
use commands::linkage_lib::{disable, enable, LinkageLibState};
use std::sync::{Arc, Mutex};

fn main() {
    let common_config = Arc::new(Mutex::new(config::config().unwrap()));
    logging::Logger::new(
        common_config
            .lock()
            .unwrap()
            .cockpit()
            .cockpit_backend_logger_address()
            .port,
    )
    .start();

    tauri::Builder::default()
        .manage(LinkageLibState::new())
        .manage(GamepadState::new())
        .manage(ConfigState::new(common_config))
        .invoke_handler(tauri::generate_handler![
            enable,
            disable,
            set_cockpit_config,
            get_config,
            start_gamepad_event_listener,
            start_gauge_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
