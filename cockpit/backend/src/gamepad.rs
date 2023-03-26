use std::io::Write;
use std::mem;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};

use gilrs::{
    EventType::{AxisChanged, ButtonChanged, Connected, Disconnected},
    GamepadId, Gilrs,
};

use common::messages::{BackendToLinkage, Message};

enum EventType {
    ButtonChanged,
    AxisChanged,
    Connected,
    Disconnected,
}

impl From<EventType> for u8 {
    fn from(value: EventType) -> Self {
        match value {
            EventType::ButtonChanged => 0,
            EventType::AxisChanged => 1,
            EventType::Connected => 2,
            EventType::Disconnected => 3,
        }
    }
}

pub(crate) fn channel(sender: Sender<BackendToLinkage>) {
    let mut gilrs = Gilrs::new().unwrap();

    fn gamepad_id_into_u8(gamepad_id: GamepadId) -> u8 {
        unsafe { mem::transmute_copy::<GamepadId, usize>(&gamepad_id) as u8 }
    }

    let send = |gamepad_id: GamepadId, event_type: EventType, control: u8, value: u8| {
        let gamepad_id = gamepad_id_into_u8(gamepad_id);

        let message = BackendToLinkage::GamepadInputEvent {
            gamepad_id,
            event_type: event_type.into(),
            control,
            value,
        };

        sender.send(message).unwrap_or_else(|error| {
            eprintln!("ERROR: Failed to send GamepadInputEvent over channel: {error}")
        })
    };

    loop {
        while let Some(gilrs::Event { id, event, time: _ }) = gilrs.next_event() {
            match event {
                ButtonChanged(button, value, _code) => send(
                    id,
                    EventType::ButtonChanged,
                    button as u8,
                    (value.clamp(0.0, 1.0) * 255.0) as u8,
                ),
                AxisChanged(axis, value, _code) => send(
                    id,
                    EventType::AxisChanged,
                    axis as u8,
                    (((value.clamp(-1.0, 1.0) + 1.0) / 2.0) * 255.0) as u8,
                ),
                Connected => send(id, EventType::Connected, 0, 0),
                Disconnected => send(id, EventType::Disconnected, 0, 0),
                _ => {}
            }
        }
    }
}

pub(crate) fn handle_input(linkage_stream: &TcpStream, receiver: Receiver<BackendToLinkage>) {
    while let Ok(message) = receiver.recv() {
        let mut stream = linkage_stream;
        stream
            .write(&message.to_bytes())
            .expect("should be able to write to TCP connection with Linkage lib");
        eprintln!("[LinkageConnection] Sent message: {message:?}.");
    }
}
