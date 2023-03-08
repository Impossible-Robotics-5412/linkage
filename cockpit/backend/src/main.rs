use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use common::messages::{
    BackendToFrontendMessage, BackendToRuntimeMessage, Bytes, Message, RuntimeToBackendMessage,
};

fn main() -> io::Result<()> {
    let config = common::config::config().unwrap();
    let address = format!("0.0.0.0:{}", config.cockpit_backend().port());
    ws::listen(address, move |frontend| {
        let runtime_stream =
            TcpStream::connect(config.cockpit_backend().runtime_address().to_string())
                .expect("should connect to runtime");
        // FIXME: We still crash when Runtime is not found or isn't running. see issue #24

        eprintln!(
            "Connected to Runtime on address {}.",
            runtime_stream.local_addr().unwrap()
        );

        move |msg| {
            let mut runtime_stream = runtime_stream.try_clone().unwrap();

            eprintln!("Received message from frontend: {msg:?}");

            match msg {
                ws::Message::Text(_) => todo!(),
                ws::Message::Binary(buffer) => {
                    if buffer.len() != 8 {
                        eprintln!(
                            "Binary data should have 8 bytes, but found {}",
                            buffer.len()
                        );
                        return Ok(());
                    }

                    let instruction = buffer[0];
                    match instruction {
                        0x00 => {
                            enable_linkage(&mut runtime_stream)?;
                            frontend.send(ws::Message::Binary(
                                BackendToFrontendMessage::Enabled.to_bytes().to_vec(),
                            ))?;
                        }
                        0x01 => {
                            disable_linkage(&mut runtime_stream)?;
                            frontend.send(ws::Message::Binary(
                                BackendToFrontendMessage::Disabled.to_bytes().to_vec(),
                            ))?;
                        }
                        _ => eprintln!("Unknown Instuction {instruction}"),
                    }
                }
            }

            Ok(())
        }
    })
    .unwrap();

    Ok(())
}

fn enable_linkage(runtime_stream: &mut TcpStream) -> io::Result<()> {
    runtime_stream.write(&BackendToRuntimeMessage::Enable.to_bytes())?;

    let mut buffer = Bytes::default();
    runtime_stream.read_exact(&mut buffer)?;

    let msg = RuntimeToBackendMessage::try_from(buffer).expect("should be a valid message");

    match msg {
        RuntimeToBackendMessage::Enabled => {
            // FIXME: Start sending controller input events to linkage.
            eprintln!("linkage has been enabled")
        }
        _ => unreachable!(
            "runtime should not send back disabled message after receiving an enable message"
        ),
    }

    Ok(())
}

fn disable_linkage(runtime_stream: &mut TcpStream) -> io::Result<()> {
    runtime_stream.write(&BackendToRuntimeMessage::Disable.to_bytes())?;

    let mut buffer = Bytes::default();
    runtime_stream.read_exact(&mut buffer)?;

    let msg = RuntimeToBackendMessage::try_from(buffer).expect("should be a valid message");

    match msg {
        RuntimeToBackendMessage::Disabled => {
            eprintln!("linkage has been disabled");
        }
        _ => unreachable!(
            "runtime should not send back enabled message after receiving a disable message"
        ),
    }

    Ok(())
}
