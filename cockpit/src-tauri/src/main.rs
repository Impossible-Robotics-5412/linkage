// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod gamepad;

use crate::commands::config::ConfigState;
use commands::config::config;
use commands::gauge::start_gauge_connection;
use commands::linkage_lib::{disable, enable, LinkageLibState};
use std::sync::Arc;

fn main() {
    let common_config = Arc::new(config::config().unwrap());
    logging::Logger::new(
        common_config
            .cockpit()
            .cockpit_backend_logger_address()
            .port,
    )
    .start();

    let gamepad_event_bus = gamepad::start_event_listener();

    tauri::Builder::default()
        .manage(LinkageLibState::new(gamepad_event_bus))
        .manage(ConfigState::new(common_config))
        .invoke_handler(tauri::generate_handler![
            enable,
            disable,
            config,
            start_gauge_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
