use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc::{channel, Receiver, Sender},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use bus::{Bus, BusReader};
use common::{
    config::Config,
    messages::{CockpitToLinkage, Message},
};
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

#[tauri::command]
pub fn enable<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, LinkageLibState>,
) -> Result<(), String> {
    let config = common::config::config().map_err(|err| format!("Failed to load config: {err}"))?;
    log::debug!("Received enable command");

    thread::spawn({
        let disable_message_receiver = state.disabled_message_receiver.clone();
        let gamepad_event_bus_rx = state.gamepad_event_bus.lock().unwrap().add_rx();

        move || {
            let socket_address = config.cockpit().linkage_socket_address();
            log::debug!("Starting Linkage-lib socket on '{socket_address}'...");
            let mut socket = TcpStream::connect(socket_address.to_string()).unwrap();

            // Make sure the service has started
            socket.read(&mut [0]).unwrap();

            log::debug!("Started Linkage-lib socket.");
            app.emit_all(
                EVENT_LINKAGE_LIB_STATE_CHANGE,
                LinkageLibStateChange::Enabled,
            )
            .unwrap();

            thread::spawn(move || start_linkage_lib_communication(config, gamepad_event_bus_rx));

            block_until_disable(&mut socket, &disable_message_receiver);

            app.emit_all(
                EVENT_LINKAGE_LIB_STATE_CHANGE,
                LinkageLibStateChange::Disabled,
            )
            .unwrap();

            log::debug!("Closed Linkage-lib service socket.");
        }
    });

    Ok(())
}

#[tauri::command]
pub fn disable(state: tauri::State<'_, LinkageLibState>) -> Result<(), String> {
    log::debug!("Received disable command");

    state
        .disabled_message_sender
        .lock()
        .unwrap()
        .send(())
        .map_err(|err| format!("Failed to send disable message: {err}"))
}

fn start_linkage_lib_communication(
    config: Config,
    mut gamepad_event_bus_rx: BusReader<Option<CockpitToLinkage>>,
) {
    let linkage_lib_address = config.cockpit().linkage_lib_address();
    log::debug!("Starting Linkage-lib communication on '{linkage_lib_address}'.");

    let mut handle_gamepad_events = |linkage_communication_stream: &mut TcpStream| {
        loop {
            match gamepad_event_bus_rx.recv() {
                Ok(Some(message)) => {
                    if let Err(err) = linkage_communication_stream.write(&message.to_bytes()) {
                        match err.kind() {
                            ErrorKind::BrokenPipe => {
                                // If the pipe is broken, we can't use this connection anymore
                                // so let's just break out of the loop, so we can disconnect this stream and close this thread.
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
                err => {
                    log::error!("Invalid message from gamepad event bus: {:?}", err);
                }
            }
        }
    };

    match TcpStream::connect(linkage_lib_address.to_string()) {
        Ok(mut linkage_communication_stream) => {
            // We are connected to the Linkage communication socket.
            handle_gamepad_events(&mut linkage_communication_stream);
        }
        Err(err) => log::error!(
            "Failed to connect to linkage on address '{}': {}",
            linkage_lib_address.to_string(),
            err
        ),
    }
}

fn block_until_disable(socket: &mut TcpStream, disable_message_receiver: &Mutex<Receiver<()>>) {
    socket
        .set_read_timeout(Some(Duration::from_millis(100)))
        .unwrap();

    // Keep blocking until the socket closes.
    loop {
        // Check if a disable message has been sent from the frontend.
        match disable_message_receiver.lock().unwrap().try_recv() {
            // Received disabled message from frontend.
            Ok(()) => {
                log::debug!("Closing Linakge socket: Disable message received");
                break;
            }
            // Didn't receive a disable message from the frontend,
            // so let's see if the connection has been closed.
            Err(_) => {
                match socket.read_exact(&mut vec![0]) {
                    Err(err) if err.kind() == ErrorKind::UnexpectedEof => {
                        // The socket has been closed.
                        log::debug!(
                            "Closing Linkage socket: Linkage socket received UnexpectedEof"
                        );
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}
