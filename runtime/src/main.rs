mod processes;
mod state;

use std::io::{self, Read};
use std::net::TcpListener;

use common::messages::{BackendToRuntimeMessage, Bytes};
use state::State;

fn main() -> io::Result<()> {
    let config = common::config::config().expect("should get config");

    let address = format!("0.0.0.0:{}", config.runtime().port());
    let listener = TcpListener::bind(address).expect("address should be valid");
    eprintln!("Started Listening on {}", listener.local_addr()?);

    let (alrm_signal_sender, alrm_signal_receiver) = crossbeam::channel::unbounded();
    processes::handle_alrm_signal(alrm_signal_sender);

    for backend in listener.incoming() {
        let backend = backend?;

        let mut buffer = Bytes::default();
        let mut state = State::new(backend.try_clone()?, alrm_signal_receiver.clone());
        loop {
            if backend.try_clone()?.read_exact(&mut buffer).is_err() {
                break;
            };
            let peer = backend.peer_addr()?;
            eprintln!("Connection established with {peer}");

            let msg = match BackendToRuntimeMessage::try_from(buffer) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Unknown message: {e:?}");
                    continue;
                }
            };

            eprintln!("Received message: {buffer:?} ({msg:?})");
            match msg {
                BackendToRuntimeMessage::Enable => state.enable(config.runtime()),
                BackendToRuntimeMessage::Disable => state.disable()?,
            }

            eprintln!("Connection with {peer} closed.");
        }
    }

    Ok(())
}
