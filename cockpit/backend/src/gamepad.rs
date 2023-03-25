use std::io::Write;
use std::mem;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};

use gilrs::{
    EventType::{AxisChanged, ButtonChanged, Connected, Disconnected},
    GamepadId, Gilrs,
};

use common::messages::{BackendToLinkage, Message};

pub(crate) fn channel(sender: Sender<BackendToLinkage>) {
    let mut gilrs = Gilrs::new().unwrap();

    fn gamepad_id_into_u8(gamepad_id: GamepadId) -> u8 {
        unsafe { mem::transmute_copy::<GamepadId, usize>(&gamepad_id) as u8 }
    }

    let send = |gamepad_id: GamepadId, code: u32, value: u8| {
        let value = value;
        let gamepad_id = gamepad_id_into_u8(gamepad_id);
        let code_page = (code >> 16) as u8;
        let code_usage = (code & 0xff) as u8;

        let message = BackendToLinkage::GamepadInputEvent {
            value,
            id: gamepad_id,
            code_page,
            code_usage,
        };

        sender.send(message).unwrap_or_else(|error| {
            eprintln!("ERROR: Failed to send GamepadInputEvent over channel: {error}")
        })
    };

    loop {
        while let Some(gilrs::Event { id, event, time: _ }) = gilrs.next_event() {
            match event {
                ButtonChanged(_button, value, code) => {
                    send(id, code.into_u32(), (value.clamp(0.0, 1.0) * 255.0) as u8)
                }
                AxisChanged(_axis, value, code) => send(
                    id,
                    code.into_u32(),
                    (((value.clamp(-1.0, 1.0) + 1.0) / 2.0) * 255.0) as u8,
                ),
                Connected | Disconnected => {
                    let value = match event {
                        Connected => 1,
                        Disconnected => 0,
                        _ => unreachable!(),
                    };

                    send(id, 0xff, value);
                }
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
