use std::{
    io::{self, Write},
    net::TcpStream,
};

use crate::gamepad::GamepadInputEvent;

type MessageBytes = [u8; 8];

#[derive(Debug, Clone, Copy)]
pub(crate) enum LinkageInstruction {
    GamepadEvent(GamepadInputEvent),
}

impl LinkageInstruction {
    fn instruction(self) -> u8 {
        match self {
            LinkageInstruction::GamepadEvent(_) => 0x00,
        }
    }

    fn to_bytes(self) -> MessageBytes {
        let mut bytes = MessageBytes::default();
        bytes[0] = self.instruction();

        match self {
            LinkageInstruction::GamepadEvent(event) => {
                bytes[1] = event.value;
                bytes[2] = event.gamepad_id;
                bytes[3] = event.code_page;
                bytes[4] = event.code_usage;

                bytes
            }
        };

        bytes
    }
}

pub(crate) fn send_instruction(
    mut stream: &TcpStream,
    instruction: LinkageInstruction,
) -> io::Result<()> {
    stream.write(&instruction.to_bytes())?;
    eprintln!("[LinkageConnection] Sent instruction: {instruction:?}.");
    Ok(())
}
