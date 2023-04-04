use std::{
    io::{ErrorKind, Read},
    net::TcpStream,
    sync::Mutex,
    thread,
    time::Duration,
};

use bus::{Bus, BusReader};

pub struct LinkageLibSocketState {
    bus: Mutex<Bus<Option<()>>>,
}

impl Default for LinkageLibSocketState {
    fn default() -> Self {
        Self {
            bus: Mutex::new(Bus::new(1)),
        }
    }
}

#[tauri::command]
pub async fn enable(state: tauri::State<'_, LinkageLibSocketState>) -> Result<(), String> {
    let mut bus_rx = state.bus.lock().unwrap().add_rx();

    thread::spawn(move || {
        start_socket(&mut bus_rx);
    });

    Ok(())
}

#[tauri::command]
pub async fn disable(state: tauri::State<'_, LinkageLibSocketState>) -> Result<(), String> {
    stop_socket(&mut state.bus.lock().unwrap());
    Ok(())
}

fn start_socket(bus_rx: &mut BusReader<Option<()>>) {
    log::debug!("Starting Linkage-lib socket...");

    let mut socket = TcpStream::connect("raspberrypi.local:9999").unwrap();

    // Make sure the service has started
    socket.read(&mut [0]).unwrap();

    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    log::debug!("Started Linkage-lib socket.");

    // FIXME: Tell frontend the socket has been started.

    // Keep block until the socket closes.
    loop {
        match bus_rx.try_recv() {
            Ok(val) => match val {
                None => break,
                _ => {}
            },
            Err(_) => match socket.read(&mut [0]) {
                Ok(0) => break,
                Err(err) => match err.kind() {
                    ErrorKind::TimedOut | ErrorKind::WouldBlock => {}
                    _ => break,
                },
                _ => {}
            },
        }
    }

    log::debug!("Closed Linkage-lib service socket.");
}

fn stop_socket(bus: &mut Bus<Option<()>>) {
    bus.broadcast(None);
}
