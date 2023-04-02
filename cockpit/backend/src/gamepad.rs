use bus::{Bus, BusReader};
use common::messages::{BackendToLinkage, Message};

use std::mem;
use std::net::TcpStream;

use std::sync::Mutex;
use std::{io::Write, sync::Arc};

use gilrs::{
    EventType::{AxisChanged, ButtonChanged, Connected, Disconnected},
    GamepadId, Gilrs,
};
use log::debug;

#[repr(u8)]
enum EventType {
    ButtonChanged = 0,
    AxisChanged = 1,
    Connected = 2,
    Disconnected = 3,
}

pub(crate) fn start_event_listener(linkage_bus: Arc<Mutex<Bus<BackendToLinkage>>>) {
    let mut gilrs = Gilrs::new().unwrap();

    fn gamepad_id_into_u8(gamepad_id: GamepadId) -> u8 {
        unsafe { mem::transmute_copy::<GamepadId, usize>(&gamepad_id) as u8 }
    }

    let send = |gamepad_id: GamepadId, event_type: EventType, control: u8, value: u8| {
        let gamepad_id = gamepad_id_into_u8(gamepad_id);

        let message = BackendToLinkage::GamepadInputEvent {
            gamepad_id,
            event_type: event_type as u8,
            control,
            value,
        };

        linkage_bus.lock().unwrap().broadcast(message);
    };

    loop {
        if let Some(gilrs::Event { id, event, time: _ }) = gilrs.next_event() {
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
            continue;
        }
    }
}

pub(crate) fn handle_input(
    linkage_stream: &TcpStream,
    linkage_bus_rx: &mut BusReader<BackendToLinkage>,
) {
    while let Ok(message) = linkage_bus_rx.recv() {
        let mut stream = linkage_stream;
        stream
            .write(&message.to_bytes())
            .expect("should be able to write to TCP connection with Linkage lib");
        debug!("Sent message: {message:?}");
    }
}
