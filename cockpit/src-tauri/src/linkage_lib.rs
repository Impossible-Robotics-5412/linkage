use std::{
    io::{self, ErrorKind, Read},
    net::TcpStream,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use tauri::{Manager, Runtime};

#[derive(serde::Serialize, Clone, Copy)]
enum LinkageLibStateChange {
    Enabled,
    Disabled,
}

enum ChannelMessage {
    Disable,
}

pub struct LinkageLibSocketState {
    sender: Mutex<Sender<ChannelMessage>>,
    receiver: Arc<Mutex<Receiver<ChannelMessage>>>,
}

impl Default for LinkageLibSocketState {
    fn default() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender: Mutex::new(sender),
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
}

#[tauri::command]
pub async fn enable<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, LinkageLibSocketState>,
) -> Result<(), String> {
    start_socket(app, state.receiver.clone()).map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn disable(state: tauri::State<'_, LinkageLibSocketState>) -> Result<(), String> {
    stop_socket(state.inner());
    Ok(())
}

fn start_socket<R: Runtime>(
    app: tauri::AppHandle<R>,
    receiver: Arc<Mutex<Receiver<ChannelMessage>>>,
) -> io::Result<()> {
    log::debug!("Starting Linkage-lib socket...");

    let mut socket = TcpStream::connect("raspberrypi.local:9999")?;

    // Make sure the service has started
    socket.read(&mut [0])?;

    log::debug!("Started Linkage-lib socket.");
    app.emit_all("linkage-lib-state-change", LinkageLibStateChange::Enabled)
        .unwrap();

    socket.set_read_timeout(Some(Duration::from_millis(100)))?;
    thread::spawn({
        move || {
            // Keep block until the socket closes.
            loop {
                match receiver.lock().unwrap().try_recv() {
                    // Check if a disable message has been sent from the frontend.
                    Ok(val) => match val {
                        ChannelMessage::Disable => break,
                    },
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

            app.emit_all("linkage-lib-state-change", LinkageLibStateChange::Disabled)
                .unwrap();

            log::debug!("Closed Linkage-lib service socket.");
        }
    });

    Ok(())
}

fn stop_socket(state: &LinkageLibSocketState) {
    state
        .sender
        .lock()
        .unwrap()
        .send(ChannelMessage::Disable)
        .unwrap();
}
