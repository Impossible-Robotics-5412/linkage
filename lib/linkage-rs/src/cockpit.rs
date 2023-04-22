use config::AddressPort;
use messaging::{Bytes, CockpitToLinkage};
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};

use crate::state::RobotStateHandle;

pub(crate) fn start_listener(state: RobotStateHandle, port: &AddressPort) -> io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;

    std::thread::spawn(move || {
        for cockpit_stream in listener.incoming() {
            log::info!("Cockpit connected!");
            handle_cockpit_client(cockpit_stream.unwrap(), state.clone());
        }
    });

    Ok(())
}

fn handle_cockpit_client(mut cockpit_stream: TcpStream, state: RobotStateHandle) {
    let mut message_bytes = Bytes::default();
    while let Ok(()) = cockpit_stream.read_exact(&mut message_bytes) {
        match CockpitToLinkage::try_from(message_bytes) {
            Ok(message) => state
                .lock()
                .unwrap()
                .gamepad_manager
                .handle_cockpit_message(message),
            Err(error) => log::error!("Failed to parse bytes into message: {error}"),
        }
    }
}
