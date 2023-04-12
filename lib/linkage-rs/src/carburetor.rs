use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::mpsc::Receiver;

use common::messages::{Bytes, LinkageToCarburetor};

pub(crate) fn open_connection(message_receiver: Receiver<LinkageToCarburetor>) -> io::Result<()> {
    // FIXME: Get this address from the config.
    let address = "raspberrypi.local:48862";
    let mut stream = TcpStream::connect(address)?;

    log::info!("Opened connection with Carburetor on '{address}'");

    // FIXME: This thread does not automatically close when we call shutdown on the Robot.
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
