use common::logging::setup_logger;
use common::messages::{BackendToFrontendMessage, Bytes, FrontendToBackendMessage, Message};

use std::error::Error;
use std::io;

use log::{debug, error, info};

mod gamepad;
mod linkage_lib;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger(7642)?;

    let config = common::config::config().unwrap();
    let address = format!("0.0.0.0:{}", config.cockpit_backend().port());

    ws::listen(&address, move |frontend| {
        info!("Connected with frontend!");

        move |msg| {
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
                            if let Err(error) = enable_linkage() {
                                error!("Failed to enable linkage: {error}");
                                return Ok(());
                            };

                            frontend.send(ws::Message::Binary(
                                BackendToFrontendMessage::Enabled.to_bytes().to_vec(),
                            ))?;
                        }
                        FrontendToBackendMessage::Disable => {
                            if let Err(error) = disable_linkage() {
                                error!("Failed to enable linkage: {error}");
                                return Ok(());
                            };

                            frontend.send(ws::Message::Binary(
                                BackendToFrontendMessage::Disabled.to_bytes().to_vec(),
                            ))?;
                        }
                    }
                }
            }

            debug!("Handled frontend message!");

            Ok(())
        }
    })
    .expect(format!("should start listening on {address}").as_str());

    Ok(())
}

fn enable_linkage() -> io::Result<()> {
    todo!("Implement");
}

fn disable_linkage() -> io::Result<()> {
    todo!("Implement");
}
