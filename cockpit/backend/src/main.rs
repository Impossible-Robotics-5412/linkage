use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

use common::messages::{
    BackendToFrontendMessage, BackendToRuntimeMessage, Bytes, FrontendToBackendMessage, Message,
    RuntimeToBackendMessage,
};

mod gamepad;
mod linkage_lib;

fn main() -> io::Result<()> {
    let config = common::config::config().unwrap();
    let address = format!("0.0.0.0:{}", config.cockpit_backend().port());

    ws::listen(&address, move |frontend| {
        let linkage_lib_address = config.cockpit_backend().linkage_lib_address().to_string();

        let runtime_stream =
            TcpStream::connect(config.cockpit_backend().runtime_address().to_string())
                .expect("should connect to runtime");

        println!(
            "Connected to Runtime on address {}.",
            runtime_stream.local_addr().unwrap()
        );

        move |msg| {
            let mut runtime_stream = runtime_stream.try_clone().unwrap();

            match msg {
                ws::Message::Text(_) => todo!(),
                ws::Message::Binary(buffer) => {
                    if buffer.len() != 8 {
                        println!(
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
                            println!("Unknown message: {e:?}");
                            return Ok(());
                        }
                    };

                    println!("Received message: {msg:?} {buffer:?}");
                    match msg {
                        FrontendToBackendMessage::Enable => {
                            if let Err(error) =
                                enable_linkage(&mut runtime_stream, linkage_lib_address.clone())
                            {
                                println!(
                                    "Connection with runtime broke. ({error})\nTo connect  again, \
                                    restart runtime and reconnect cockpit-backend to runtime"
                                );
                                return Ok(());
                            };
                            frontend.send(ws::Message::Binary(
                                BackendToFrontendMessage::Enabled.to_bytes().to_vec(),
                            ))?;
                        }
                        FrontendToBackendMessage::Disable => {
                            if let Err(error) = disable_linkage(&mut runtime_stream) {
                                println!(
                                    "Connection with runtime broke. ({error})\nTo connect  again, \
                                    restart runtime and reconnect cockpit-backend to runtime"
                                );
                                return Ok(());
                            };
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
    .expect(format!("should start listening on {address}").as_str());

    Ok(())
}

fn enable_linkage(runtime_stream: &mut TcpStream, linkage_address: String) -> io::Result<()> {
    runtime_stream.write(&BackendToRuntimeMessage::Enable.to_bytes())?;

    let mut buffer = Bytes::default();
    runtime_stream.read_exact(&mut buffer)?;

    let msg = RuntimeToBackendMessage::try_from(buffer).expect("should be a valid message");

    match msg {
        RuntimeToBackendMessage::Enabled => {
            println!("Linkage has been enabled");
            linkage_lib::start_communication(linkage_address);
        }
        _ => unreachable!(
            "runtime should only send back ENABLED message after receiving an ENABLE message"
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
            println!("Linkage has been disabled");
        }
        _ => unreachable!(
            "runtime only send back DISABLED message after receiving a DISABLE message"
        ),
    }

    Ok(())
}
