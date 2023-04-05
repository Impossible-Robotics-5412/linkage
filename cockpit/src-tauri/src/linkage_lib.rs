use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc::{channel, Receiver, Sender},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use bus::Bus;
use common::messages::{CockpitToLinkage, Message};
use tauri::{Manager, Runtime};

const EVENT_LINKAGE_LIB_STATE_CHANGE: &str = "linkage-lib-state-change";

#[derive(serde::Serialize, Clone, Copy)]
enum LinkageLibStateChange {
    Enabled,
    Disabled,
}

pub struct LinkageLibState {
    gamepad_event_bus: Arc<Mutex<Bus<Option<CockpitToLinkage>>>>,
    disabled_message_sender: Mutex<Sender<()>>,
    disabled_message_receiver: Arc<Mutex<Receiver<()>>>,
}

impl LinkageLibState {
    pub fn new(gamepad_event_bus: Arc<Mutex<Bus<Option<CockpitToLinkage>>>>) -> Self {
        let (sender, receiver) = channel();
        Self {
            gamepad_event_bus,
            disabled_message_sender: Mutex::new(sender),
            disabled_message_receiver: Arc::new(Mutex::new(receiver)),
        }
    }
}

// FIXME: Handle errors
#[tauri::command]
pub fn enable<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, LinkageLibState>,
) -> Result<(), String> {
    thread::spawn({
        let receiver = state.disabled_message_receiver.clone();
        let mut gamepad_event_bus_rx = state.gamepad_event_bus.lock().unwrap().add_rx();
        move || {
            log::debug!("Starting Linkage-lib socket...");

            let mut socket = TcpStream::connect("raspberrypi.local:9999").unwrap();

            // Make sure the service has started
            socket.read(&mut [0]).unwrap();

            log::debug!("Started Linkage-lib socket.");
            app.emit_all(
                EVENT_LINKAGE_LIB_STATE_CHANGE,
                LinkageLibStateChange::Enabled,
            )
            .unwrap();

            socket
                .set_read_timeout(Some(Duration::from_millis(100)))
                .unwrap();

            thread::spawn({
                move || {
                    // Keep block until the socket closes.
                    loop {
                        match receiver.lock().unwrap().try_recv() {
                            // Check if a disable message has been sent from the frontend.
                            Ok(()) => break,
                            // Check if the socket has been disconnected.
                            Err(_) => match socket.read_exact(&mut [0]) {
                                Err(err) => match err.kind() {
                                    ErrorKind::TimedOut | ErrorKind::WouldBlock => {}
                                    _ => break,
                                },
                                _ => {}
                            },
                        }
                    }

                    app.emit_all(
                        EVENT_LINKAGE_LIB_STATE_CHANGE,
                        LinkageLibStateChange::Disabled,
                    )
                    .unwrap();

                    log::debug!("Closed Linkage-lib service socket.");
                }
            });

            thread::spawn({
                move || {
                    let mut linkage_communication_stream =
                        TcpStream::connect("raspberrypi.local:12362").unwrap();

                    while let Ok(Some(message)) = gamepad_event_bus_rx.recv() {
                        linkage_communication_stream
                            .write(&message.to_bytes())
                            .unwrap();
                    }
                }
            });
        }
    });

    Ok(())
}

#[tauri::command]
pub fn disable(state: tauri::State<'_, LinkageLibState>) -> Result<(), String> {
    state
        .disabled_message_sender
        .lock()
        .unwrap()
        .send(())
        .unwrap();

    Ok(())
}
