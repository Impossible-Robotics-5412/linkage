use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    thread,
};

use common::system_info::SystemInfo;
use tauri::Manager;

const EVENT_RECEIVED_SYSTEM_INFO: &str = "received_system_info";

#[tauri::command]
pub fn start_gauge_connection<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    thread::spawn(move || match TcpStream::connect("raspberrypi.local:4226") {
        Ok(stream) => {
            let mut buf_reader = BufReader::new(stream);

            let mut json_string = String::new();
            while buf_reader.read_line(&mut json_string).is_ok() {
                let system_info: SystemInfo = serde_json::from_str(json_string.trim()).unwrap();
                on_receive_system_info(&app, system_info).unwrap();
                json_string.clear()
            }
        }
        Err(error) => {
            log::error!("Failed to open stream with Gauge: {error}");
        }
    });

    Ok(())
}

fn on_receive_system_info<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    system_info: SystemInfo,
) -> Result<(), tauri::Error> {
    app.emit_all(EVENT_RECEIVED_SYSTEM_INFO, system_info)
}
