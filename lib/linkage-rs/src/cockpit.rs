use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};

use common::messages::{Bytes, CockpitToLinkage};

use crate::gamepad::GamepadData;
use crate::state::RobotStateHandle;

pub fn start_cockpit_listener(state: RobotStateHandle) -> io::Result<()> {
    // FIXME: Get this port from the config.
    let port = 12362;
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;

    // FIXME: This thread does not automatically close when we call shutdown on the Robot.
    std::thread::spawn(move || {
        for cockpit_stream in listener.incoming() {
            handle_cockpit_client(cockpit_stream.unwrap(), state.clone());
        }
    });

    Ok(())
}

fn handle_cockpit_client(mut cockpit_stream: TcpStream, state: RobotStateHandle) {
    let mut message_bytes = Bytes::default();
    while let Ok(()) = cockpit_stream.read_exact(&mut message_bytes) {
        match CockpitToLinkage::try_from(message_bytes) {
            Ok(message) => handle_cockpit_message(message, state.clone()),
            Err(error) => log::error!("Failed to parse bytes into message: {error}"),
        }
    }
}

fn handle_cockpit_message(message: CockpitToLinkage, state: RobotStateHandle) {
    match message {
        CockpitToLinkage::GamepadInputEvent {
            gamepad_id,
            event_type,
            control,
            value,
        } => {
            let mut state = state.lock().unwrap();

            let gamepad = state
                .gamepads
                .entry(gamepad_id)
                .or_insert(GamepadData::default());

            // FIXME: Handle Connect and Disconnect EventType

            gamepad
                .handle_cockpit_message(event_type, control, value)
                .unwrap();
        }
    }
}
