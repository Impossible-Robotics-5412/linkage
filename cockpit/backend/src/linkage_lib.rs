use std::net::TcpStream;
use std::sync::mpsc::channel;

use crate::gamepad;

pub(crate) fn start_communication(linkage_address: String) {
    // NOTE: If we want to send controller info to frontend, we should start listening for
    //       Controller input immediately after starting, and not just when enabling Linkage-lib.
    let (linkage_tx, linkage_rx) = channel();
    std::thread::spawn(move || gamepad::channel(linkage_tx));

    std::thread::spawn(move || {
        let linkage_stream =
            TcpStream::connect(linkage_address).expect("should connect to Linkage-lib");
        gamepad::handle_input(&linkage_stream, linkage_rx);
    });
}
