use std::net::TcpStream;



use bus::BusReader;
use common::messages::BackendToLinkage;

use crate::gamepad;

pub(crate) fn start_communication(
    linkage_address: String,
    linkage_bus_rx: &mut BusReader<BackendToLinkage>,
) {
    let linkage_stream =
        TcpStream::connect(linkage_address).expect("should connect to Linkage-lib");
    gamepad::handle_input(&linkage_stream, linkage_bus_rx);
}
