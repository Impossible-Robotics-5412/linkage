use std::io::ErrorKind;
use std::time::Duration;
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    thread,
};

use system_info::{decode_system_info_from_string, SystemInfo};
use tauri::Manager;

const EVENT_RECEIVED_SYSTEM_INFO: &str = "received_system_info";

#[tauri::command]
pub fn start_gauge_connection<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    let config = config::config().map_err(|err| format!("Failed to  load config: {err}"))?;
    thread::spawn(
        move || match TcpStream::connect(config.cockpit().gauge_address().to_string()) {
            Ok(stream) => {
                stream
                    .set_read_timeout(Some(Duration::from_millis(2000)))
                    .unwrap();

                let mut buf_reader = BufReader::new(stream);

                loop {
                    let mut json_string = String::new();
                    match buf_reader.read_line(&mut json_string) {
                        Ok(_) => {
                            on_receive_system_info(
                                &app,
                                decode_system_info_from_string(json_string),
                            )
                            .unwrap();
                        }
                        Err(error) if error.kind() != ErrorKind::WouldBlock => {
                            log::error!(
                                "Failed to read message from Gauge TcpStream: {}, {}",
                                error.kind(),
                                error
                            )
                        }
                        _ => {}
                    }
                }
            }
            Err(error) => {
                log::error!("Failed to open stream with Gauge: {error}");
            }
        },
    );

    Ok(())
}

fn on_receive_system_info<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    system_info: SystemInfo,
) -> Result<(), tauri::Error> {
    app.emit_all(EVENT_RECEIVED_SYSTEM_INFO, system_info)
}
