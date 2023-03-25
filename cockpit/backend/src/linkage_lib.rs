use std::{
    io::{self, Write},
    net::TcpStream,
    sync::mpsc::channel,
};

use crate::gamepad::{self, GamepadInputEvent};

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

pub(crate) fn start_communication() {
    // NOTE: If we want to send controller info to frontend, we should start listening for
    //       Controller input immediately after starting, and not just when enabling Linkage-lib.
    let (linkage_tx, linkage_rx) = channel();
    std::thread::spawn(move || gamepad::channel(linkage_tx));

    std::thread::spawn(move || {
        // FIXME: Use address from settings
        let linkage_stream = TcpStream::connect("0.0.0.0:12362").unwrap();

        gamepad::handle_input(&linkage_stream, linkage_rx);
    });
}
