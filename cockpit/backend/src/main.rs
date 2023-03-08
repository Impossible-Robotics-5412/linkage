use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use common::messages::{
    BackendToFrontendMessage, BackendToRuntimeMessage, Bytes, FrontendToBackendMessage, Message,
    RuntimeToBackendMessage,
};

fn main() -> io::Result<()> {
    let config = common::config::config().unwrap();
    let address = format!("0.0.0.0:{}", config.cockpit_backend().port());
    ws::listen(address, move |frontend| {
        // FIXME: We still crash when Runtime is not found or isn't running. see issue #24
        let runtime_stream =
            TcpStream::connect(config.cockpit_backend().runtime_address().to_string())
                .expect("should connect to runtime");

        eprintln!(
            "Connected to Runtime on address {}.",
            runtime_stream.local_addr().unwrap()
        );

        move |msg| {
            let mut runtime_stream = runtime_stream.try_clone().unwrap();

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

                    let buffer: Bytes = buffer
                        .try_into()
                        .expect("should be able to convert 8 byte Vec<u8> to Bytes");

                    let msg = match FrontendToBackendMessage::try_from(buffer) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("Unknown message: {e:?}");
                            return Ok(());
                        }
                    };

                    eprintln!("Received message: {msg:?} {buffer:?}");
                    match msg {
                        FrontendToBackendMessage::Enable => {
                            enable_linkage(&mut runtime_stream)?;
                            frontend.send(ws::Message::Binary(
                                BackendToFrontendMessage::Enabled.to_bytes().to_vec(),
                            ))?;
                        }
                        FrontendToBackendMessage::Disable => {
                            disable_linkage(&mut runtime_stream)?;
                            frontend.send(ws::Message::Binary(
                                BackendToFrontendMessage::Disabled.to_bytes().to_vec(),
                            ))?;
                        }
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
            eprintln!("Linkage has been enabled")
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
            eprintln!("Linkage has been disabled");
        }
        _ => unreachable!(
            "runtime should not send back enabled message after receiving a disable message"
        ),
    }

    Ok(())
}
