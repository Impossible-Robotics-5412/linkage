use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::mpsc::Receiver;

use common::messages::{Bytes, LinkageToCarburetor};
use config::Address;

pub(crate) fn open_connection(
    message_receiver: Receiver<LinkageToCarburetor>,
    address: &Address,
) -> io::Result<()> {
    let mut stream = TcpStream::connect(address.to_string())?;

    log::info!("Opened connection with Carburetor on '{address}'");

    std::thread::spawn(move || loop {
        match message_receiver.recv() {
            Ok(message) => {
                let bytes: Bytes = message.into();
                if let Err(error) = stream.write(&bytes) {
                    log::error!("Failed to write message to Carburetor stream: {error}");
                }
            }
            Err(error) => log::error!("Failed to receive LinkageToCarburetor message: {error}"),
        }
    });

    Ok(())
}
