use common::logging::setup_logger;
use common::messages::{
    BackendToFrontendMessage, BackendToRuntimeMessage, Bytes, FrontendToBackendMessage, Message,
    RuntimeToBackendMessage,
};

use std::error::Error;
use std::io::{self, Read, Write};
use std::net::TcpStream;

use log::{debug, error, info};

mod gamepad;
mod linkage_lib;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger(7642)?;

    let config = common::config::config().unwrap();
    let address = format!("0.0.0.0:{}", config.cockpit_backend().port());

    ws::listen(&address, move |frontend| {
        let linkage_lib_address = config.cockpit_backend().linkage_lib_address().to_string();

        let runtime_stream =
            TcpStream::connect(config.cockpit_backend().runtime_address().to_string())
                .expect("should connect to runtime");

        info!(
            "Connected to Runtime on address {}.",
            runtime_stream.local_addr().unwrap()
        );

        move |msg| {
            let mut runtime_stream = runtime_stream.try_clone().unwrap();

            match msg {
                ws::Message::Text(_) => todo!(),
                ws::Message::Binary(buffer) => {
                    if buffer.len() != 8 {
                        error!(
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
                            error!("Unknown message: {e:?}");
                            return Ok(());
                        }
                    };

                    debug!("Received message: {msg:?} {buffer:?}");
                    match msg {
                        FrontendToBackendMessage::Enable => {
                            if let Err(error) =
                                enable_linkage(&mut runtime_stream, linkage_lib_address.clone())
                            {
                                error!(
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
                                error!(
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
            info!("Linkage has been enabled");
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
            info!("Linkage has been disabled");
        }
        _ => unreachable!(
            "runtime only send back DISABLED message after receiving a DISABLE message"
        ),
    }

    Ok(())
}
