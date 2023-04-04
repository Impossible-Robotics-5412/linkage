// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod linkage_lib;

use linkage_lib::{disable, enable, LinkageLibSocketState};

fn main() {
    common::logging::setup_logger(7642).expect("should be able to start logger");

    tauri::Builder::default()
        .manage(LinkageLibSocketState::default())
        .invoke_handler(tauri::generate_handler![enable, disable])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
