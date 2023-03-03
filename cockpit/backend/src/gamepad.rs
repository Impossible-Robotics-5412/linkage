use gilrs::{
    EventType::{AxisChanged, ButtonChanged, Connected, Disconnected},
    GamepadId, Gilrs,
};

use std::{
    mem,
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};

use crate::linkage;

use linkage::LinkageInstruction;

#[derive(Debug, Clone, Copy)]
pub(crate) struct GamepadInputEvent {
    pub value: u8,
    pub gamepad_id: u8,
    pub code_page: u8,
    pub code_usage: u8,
}

pub(crate) fn channel(sender: Sender<GamepadInputEvent>) {
    let mut gilrs = Gilrs::new().unwrap();

    fn gamepad_id_into_u8(gamepad_id: GamepadId) -> u8 {
        unsafe { mem::transmute_copy::<GamepadId, usize>(&gamepad_id) as u8 }
    }

    let send = |gamepad_id: GamepadId, code: u32, value: f32| {
        let gamepad_id = gamepad_id_into_u8(gamepad_id);
        let code_page = (code >> 16) as u8;
        let code_usage = (code & 0xff) as u8;

        sender
            .send(GamepadInputEvent {
                value: value as u8,
                gamepad_id,
                code_page,
                code_usage,
            })
            .unwrap_or_else(|error| eprintln!("ERROR: Failed to send GamepadInputEvent over channel: {error}"))
    };

    loop {
        while let Some(gilrs::Event { id, event, time: _ }) = gilrs.next_event() {
            match event {
                ButtonChanged(_button, value, code) => send(id, code.into_u32(), value.clamp(0.0, 1.0) * 255.0),
                AxisChanged(_axis, value, code) => {
                    send(id, code.into_u32(), ((value.clamp(-1.0, 1.0) + 1.0) / 2.0) * 255.0)
                }
                Connected | Disconnected => {
                    let value = match event {
                        Connected => 1.0,
                        Disconnected => 0.0,
                        _ => unreachable!(),
                    };

                    send(id, 0xff, value);
                }
                _ => {}
            }
        }
    }
}

pub(crate) fn handle_input(linkage_stream: &TcpStream, receiver: Receiver<GamepadInputEvent>) {
    while let Ok(gamepad_input) = receiver.recv() {
        linkage::send_instruction(linkage_stream, LinkageInstruction::GamepadEvent(gamepad_input)).unwrap();
    }
}
