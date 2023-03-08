mod processes;
mod state;

use state::State;
use std::io::{self, Read};
use std::net::TcpListener;

pub(crate) type MessageBytes = [u8; 8];

fn main() -> io::Result<()> {
    let config = common::config::config().expect("should get config");

    let address = format!("0.0.0.0:{}", config.runtime().port());
    let listener = TcpListener::bind(address).expect("address should be valid");
    eprintln!("Started Listening on {}", listener.local_addr()?);

    let (alrm_signal_sender, alrm_signal_receiver) = crossbeam::channel::unbounded();
    processes::handle_alrm_signal(alrm_signal_sender);

    for backend in listener.incoming() {
        let backend = backend?;

        let peer = backend.peer_addr()?;
        eprintln!("Connection established with {peer}");

        let mut state = State::new(backend.try_clone()?, alrm_signal_receiver.clone());

        let mut buffer = MessageBytes::default();
        loop {
            if backend.try_clone()?.read_exact(&mut buffer).is_err() {
                break;
            };

            eprintln!("Received message: {buffer:?}");
            match buffer[0] {
                0x00 => state.enable(config.runtime()),
                0x01 => state.disable()?,
                _ => eprintln!("Unknown message: {buffer:?}"),
            }
        }

        eprintln!("Connection with {peer} closed.");
    }

    Ok(())
}
