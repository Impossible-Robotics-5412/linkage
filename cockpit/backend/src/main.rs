use std::{
    io::{self, Write},
    net::TcpStream,
};

mod settings;

fn main() -> io::Result<()> {
    // FIXME: Implement controller input handling and sending it to linkage.
    // FIXME: Use data from config file.

    ws::listen("0.0.0.0:3012", move |frontend| {
        let runtime_stream =
            TcpStream::connect("raspberrypi:8009").expect("should connect to runtime");
        eprintln!("Connected to Runtime on address {}.", "0.0.0.0:8009");

        move |msg| {
            let mut runtime_stream = runtime_stream.try_clone().unwrap();

            eprintln!("Received message from frontend: {msg:?}");

            match msg {
                ws::Message::Text(_) => todo!(),
                ws::Message::Binary(bin) => {
                    if bin.len() != 8 {
                        eprintln!("Binary data should have 8 bytes, but found {}", bin.len());
                        return Ok(());
                    }

                    match bin.first() {
                        Some(instruction) => match instruction {
                            0x00 => {
                                enable_linkage(&mut runtime_stream)?;
                                frontend.send(ws::Message::Binary(
                                    BACKEND_TX_MESSAGE_ENABLED_LINKAGE.into(),
                                ))?;
                            }
                            0x01 => {
                                disable_linkage(&mut runtime_stream)?;
                                frontend.send(ws::Message::Binary(
                                    BACKEND_TX_MESSAGE_DISABLED_LINKAGE.into(),
                                ))?;
                            }
                            _ => {
                                eprintln!("Unknown Instuction {instruction}");
                            }
                        },
                        None => unreachable!("Binary message should have 8 bytes."),
                    };

                    Ok(())
                }
            }
        }
    })
    .unwrap();

    Ok(())
}

fn enable_linkage(runtime_stream: &mut TcpStream) -> io::Result<()> {
    runtime_stream.write(&RUNTIME_TX_MESSAGE_ENABLE_LINKAGE)?;
    // FIXME: When this returns Ok it should mean linkage has been enabled.
    //        Right now it will just tell if the message has been sent successfully to runtime,
    //        but it should really wait for a confirmation telling
    //        the linkage executable has been started on the Pi.
    Ok(())
}

fn disable_linkage(runtime_stream: &mut TcpStream) -> io::Result<()> {
    runtime_stream.write(&RUNTIME_TX_MESSAGE_DISABLE_LINKAGE)?;
    // FIXME: When this returns Ok it should mean linkage has been disabled.
    //        Right now it will just tell if the message has been sent successfully to runtime,
    //        but it should really wait for a confirmation telling
    //        the linkage executable has been killed on the Pi.
    Ok(())
}

type BackendMessage = [u8; 8];

const BACKEND_TX_MESSAGE_ENABLED_LINKAGE: BackendMessage = [0x00, 0, 0, 0, 0, 0, 0, 0];
const BACKEND_TX_MESSAGE_DISABLED_LINKAGE: BackendMessage = [0x01, 0, 0, 0, 0, 0, 0, 0];

const RUNTIME_TX_MESSAGE_ENABLE_LINKAGE: BackendMessage = [0x00, 0, 0, 0, 0, 0, 0, 0];
const RUNTIME_TX_MESSAGE_DISABLE_LINKAGE: BackendMessage = [0x01, 0, 0, 0, 0, 0, 0, 0];
