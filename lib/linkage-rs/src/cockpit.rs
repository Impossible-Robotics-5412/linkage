use std::{
    io::{self, Read},
    net::{TcpListener, TcpStream},
};

use common::messages::{Bytes, CockpitToLinkage};

pub fn start_cockpit_listener() -> io::Result<()> {
    // FIXME: Get this port from the config.
    let port = 12362;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;

    // FIXME: This thread does not automatically close when we call shutdown on the Robot.
    std::thread::spawn(move || {
        for cockpit_stream in listener.incoming() {
            handle_cockpit_client(cockpit_stream.unwrap());
        }
    });

    Ok(())
}

fn handle_cockpit_client(mut cockpit_stream: TcpStream) {
    let mut message_bytes = Bytes::default();
    while let Ok(()) = cockpit_stream.read_exact(&mut message_bytes) {
        match CockpitToLinkage::try_from(message_bytes) {
            Ok(message) => handle_cockpit_message(message),
            Err(error) => log::error!("Failed to parse bytes into message: {error}"),
        }
    }
}

fn handle_cockpit_message(message: CockpitToLinkage) {
    match message {
        CockpitToLinkage::GamepadInputEvent {
            gamepad_id,
            event_type,
            control,
            value,
        } => {
            log::info!("{gamepad_id}, {event_type}, {control}, {value}");
        }
    }
}
