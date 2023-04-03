mod gamepad;

use common::{
    logging,
    messages::{BackendToFrontendMessage, Message},
};

use std::{
    io::{self, ErrorKind, Read},
    net::TcpStream,
    sync::{self, Arc, Mutex},
    thread,
    time::Duration,
};

use websocket::{
    sync::{Client, Server},
    OwnedMessage,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::setup_logger(7642)?;

    let config = common::config::config()?;
    let address = format!("0.0.0.0:{}", config.cockpit_backend().port());

    let server = Server::bind(&address)?;
    log::info!("Started listening on {address}.");

    for request in server.filter_map(Result::ok) {
        thread::spawn(move || {
            // FIXME: Implement use_protocol shown in websocket crate example.
            let client = request.accept().unwrap();
            handle_client(client).unwrap();
        });
    }

    log::info!("Stopping.");

    Ok(())
}

fn handle_client(client: Client<TcpStream>) -> io::Result<()> {
    let ip = client.peer_addr()?;
    log::info!("Connected to Cockpit-frontend at {ip}.");

    let (mut receiver, mut sender) = client.split().unwrap();
    let message_bus = Arc::new(Mutex::new(bus::Bus::new(8)));
    let (client_sender_tx, client_sender_rx) = sync::mpsc::channel::<OwnedMessage>();

    let enable_handle = thread::spawn({
        let message_bus = message_bus.clone();
        let client_sender_tx = client_sender_tx.clone();
        move || {
            let mut rx = message_bus.lock().unwrap().add_rx();
            while let Some(message) = rx.recv().unwrap() {
                let client_sender_tx = client_sender_tx.clone();
                match message {
                    OwnedMessage::Close(_) => {
                        client_sender_tx
                            .send(OwnedMessage::Close(None))
                            .unwrap_or_else(|_| log::debug!("Failed to send CLOSED message."));
                        return;
                    }
                    OwnedMessage::Binary(buffer) => {
                        let buffer: [u8; 8] = buffer.try_into().unwrap();
                        let instruction = buffer.first().unwrap();

                        match instruction {
                            0 => enable(&mut rx, client_sender_tx),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    let disable_handle = thread::spawn({
        let message_bus = message_bus.clone();
        move || {
            let mut rx = message_bus.lock().unwrap().add_rx();
            while let Some(message) = rx.recv().unwrap() {
                let client_sender_tx = client_sender_tx.clone();
                match message {
                    OwnedMessage::Close(_) => {
                        let close_message = OwnedMessage::Close(None);
                        client_sender_tx
                            .send(close_message.clone())
                            .unwrap_or_else(|_| log::debug!("Failed to send CLOSED message."));

                        message_bus.lock().unwrap().broadcast(Some(close_message));
                        return;
                    }
                    OwnedMessage::Binary(buffer) => {
                        let buffer: [u8; 8] = buffer.try_into().unwrap();
                        let instruction = buffer.first().unwrap();

                        match instruction {
                            1 => disable(client_sender_tx),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    let sender_handle = thread::spawn(move || {
        while let Ok(message) = client_sender_rx.recv() {
            sender.send_message(&message).unwrap();
            if message.is_close() {
                return;
            }
        }
    });

    let message_bus_handle = thread::spawn(move || {
        let message_bus = message_bus.clone();
        while let Ok(message) = receiver.recv_message() {
            message_bus.lock().unwrap().broadcast(Some(message.clone()));
            if message.is_close() {
                return;
            }
        }
    });

    log::debug!("Waiting to join all thread handles...");
    disable_handle.join().unwrap();
    enable_handle.join().unwrap();
    sender_handle.join().unwrap();
    message_bus_handle.join().unwrap();
    log::debug!("Joined all thread handles.");

    log::info!("Disconnected from Cockpit-frontend at {ip}.");

    Ok(())
}

fn enable(
    message_bus_reader: &mut bus::BusReader<Option<OwnedMessage>>,
    sender: sync::mpsc::Sender<OwnedMessage>,
) {
    log::debug!("Starting Linkage-lib socket...");

    let mut socket = TcpStream::connect("raspberrypi.local:9999").unwrap();

    // Make sure the service has started
    socket.read(&mut [0]).unwrap();

    log::debug!("Started Linkage-lib socket.");
    let message_bytes = BackendToFrontendMessage::Enabled.to_bytes();
    let message = OwnedMessage::Binary(message_bytes.to_vec());

    log::debug!(
        "Sending {:?} message to Cockpit-frontend.",
        BackendToFrontendMessage::Disabled
    );
    sender
        .send(message)
        .unwrap_or_else(|_| log::debug!("Failed to send message."));

    // Keep block until the socket closes.
    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    loop {
        if let Ok(msg) = message_bus_reader.try_recv() {
            if let Some(msg) = msg {
                match msg {
                    OwnedMessage::Binary(buffer) => {
                        if buffer.first() == Some(&1u8) {
                            break;
                        }
                    }

                    OwnedMessage::Close(_) => break,
                    _ => (),
                }
            }
        }
        match socket.read(&mut [0]) {
            Ok(0) => break,
            Err(err) => match err.kind() {
                ErrorKind::TimedOut | ErrorKind::WouldBlock => {}
                _ => break,
            },
            _ => {}
        }
    }

    log::debug!("Closed Linkage-lib service socket.");

    disable(sender);
}

fn disable(sender: sync::mpsc::Sender<OwnedMessage>) {
    let message_bytes = BackendToFrontendMessage::Disabled.to_bytes();
    let message = OwnedMessage::Binary(message_bytes.to_vec());
    log::debug!(
        "Sending {:?} message to Cockpit-frontend.",
        BackendToFrontendMessage::Disabled
    );
    sender
        .send(message)
        .unwrap_or_else(|_| log::debug!("Failed to send message."));
}
