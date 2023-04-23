use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    sync::Arc,
    thread,
    time::Duration,
};

use bus::BusReader;
use tauri::{Manager, Runtime};

use config::Config;
use messaging::{CockpitToLinkage, Message};

use crate::commands::gamepad::GamepadState;

const EVENT_LINKAGE_LIB_STATE_CHANGE: &str = "linkage_lib_state_change";

#[derive(serde::Serialize, Clone, Copy)]
enum LinkageLibStateChange {
    Enabled,
    Disabled,
}

pub struct LinkageLibState {
    disabled: Arc<AtomicBool>,
}

impl LinkageLibState {
    pub fn new() -> Self {
        Self {
            disabled: Arc::new(AtomicBool::new(false)),
        }
    }
}

#[tauri::command]
pub fn enable<R: Runtime>(
    app: tauri::AppHandle<R>,
    linkage_lib_state: tauri::State<'_, LinkageLibState>,
    gamepad_state: tauri::State<'_, GamepadState>,
) -> Result<(), String> {
    let config = config::config().map_err(|err| format!("Failed to load config: {err}"))?;
    log::debug!("Received enable command");

    linkage_lib_state.disabled.store(false, Ordering::Relaxed);

    thread::spawn({
        let disabled = linkage_lib_state.disabled.clone();
        let gamepad_event_bus_rx = gamepad_state.gamepad_event_bus.lock().unwrap().add_rx();

        move || {
            let socket_address = config.cockpit().linkage_socket_address();
            log::debug!("Starting Linkage-lib socket on '{socket_address}'...");
            let mut socket = TcpStream::connect(socket_address.to_string()).unwrap();

            // Make sure the service has started
            if socket.read_exact(&mut [0]).is_err() {
                disabled.store(true, Ordering::Relaxed);
                log::warn!("Failed to check if Linkage service has been started. Disabling...");
            } else {
                log::debug!("Started Linkage-lib socket.");
                app.emit_all(
                    EVENT_LINKAGE_LIB_STATE_CHANGE,
                    LinkageLibStateChange::Enabled,
                )
                .unwrap();

                thread::spawn({
                    let disabled = disabled.clone();
                    move || {
                        start_linkage_lib_communication(config, gamepad_event_bus_rx, disabled);
                    }
                });

                block_until_disable(&mut socket, disabled.clone());
            }

            app.emit_all(
                EVENT_LINKAGE_LIB_STATE_CHANGE,
                LinkageLibStateChange::Disabled,
            )
            .unwrap();

            _ = socket.shutdown(std::net::Shutdown::Both);

            log::debug!("Closed Linkage-lib service socket.");
        }
    });

    Ok(())
}

#[tauri::command]
pub fn disable(state: tauri::State<'_, LinkageLibState>) {
    log::debug!("Received disable command");

    state.disabled.store(true, Ordering::Relaxed);
}

fn start_linkage_lib_communication(
    config: Config,
    mut gamepad_event_bus_rx: BusReader<Option<CockpitToLinkage>>,
    disabled: Arc<AtomicBool>,
) {
    let linkage_lib_address = config.cockpit().linkage_lib_address();
    log::debug!("Starting Linkage-lib communication on '{linkage_lib_address}'.");

    match TcpStream::connect(linkage_lib_address.to_string()) {
        Ok(mut linkage_communication_stream) => {
            // We are connected to the Linkage communication socket.
            loop {
                if disabled.load(Ordering::Relaxed) {
                    log::debug!("Received disable message. Breaking out of loop.");
                    break;
                }

                if let Ok(Some(message)) = gamepad_event_bus_rx.try_recv() {
                    if let Err(err) = linkage_communication_stream.write(&message.to_bytes()) {
                        match err.kind() {
                            ErrorKind::BrokenPipe => {
                                // If the pipe is broken, we can't use this connection anymore
                                // so let's just break out of the loop, so we can disconnect this stream and close this thread.
                                log::debug!("Linkage communication stream pipe is broken. Breaking out of loop.");
                                break;
                            }
                            _ => log::error!(
                                "Failed to write to Linkage communication stream on '{}': {}",
                                linkage_lib_address.to_string(),
                                err
                            ),
                        }
                    }
                }
            }

            _ = linkage_communication_stream.shutdown(std::net::Shutdown::Both)
        }
        Err(err) => log::error!(
            "Failed to connect to linkage on address '{}': {}",
            linkage_lib_address.to_string(),
            err
        ),
    }
}

fn block_until_disable(socket: &mut TcpStream, disabled: Arc<AtomicBool>) {
    socket
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();

    // Keep blocking until the socket closes.
    loop {
        // Check if a disable message has been sent from the frontend.
        if disabled.load(Ordering::Relaxed) {
            log::debug!("Closing Linakge socket: Disable message received");
            break;
        }

        // Didn't receive a disable message from the frontend,
        // so let's see if the connection has been closed.
        match socket.read_exact(&mut [0]) {
            Err(err) if err.kind() == ErrorKind::UnexpectedEof => {
                // The socket has been closed.
                log::debug!("Closing Linkage socket: Linkage socket received UnexpectedEof");
                break;
            }
            _ => {}
        }
    }
}
