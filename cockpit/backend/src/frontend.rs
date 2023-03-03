use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::{gamepad, settings::Address};

pub(crate) fn channel(runtime_stream: &mut TcpStream, receiver: Receiver<Vec<u8>>) {
    while let Ok(message) = receiver.recv() {
        runtime_stream.write(&message).unwrap();
    }
}

pub(crate) fn handle_runtime_confirmations(
    runtime_stream: &mut TcpStream,
    linkage_address: &Address,
) {
    let mut buffer = [0; 8];
    // FIXME: This will only read once for some reason. the websocket listener does receive the message. EOF bug?
    runtime_stream
        .read_exact(&mut buffer)
        .unwrap_or_else(|error| eprintln!("ERROR: Failed to read message from runtime: {error}"));

    match buffer[0] {
        0x00 => {
            let linkage_stream =
                TcpStream::connect(linkage_address.to_string()).expect("should connect to linkage");
            eprintln!("Connected to Linkage on address {}.", linkage_address);

            let (gamepad_tx, gamepad_rx) = mpsc::channel();
            thread::spawn(|| gamepad::channel(gamepad_tx));

            gamepad::handle_input(&linkage_stream, gamepad_rx);
        }
        0x01 => {
            todo!("Disconnect from linkage")
        }
        code => {
            eprintln!("ERROR: Unknown confirmation code: {code:?}");
        }
    }
}

pub(crate) fn listen(sender: Sender<Vec<u8>>) {
    ws::listen("0.0.0.0:3012", move |_out| {
        let frontend_tx = sender.clone();
        move |msg| {
            eprintln!("Received message from frontend: {msg:?}");

            match msg {
                ws::Message::Text(_) => todo!(),
                ws::Message::Binary(bin) => Ok({
                    frontend_tx.send(bin).unwrap();
                }),
            }
        }
    })
    .unwrap();
}
