mod processes;
mod state;

use common::logging::setup_logger;
use common::messages::{BackendToRuntimeMessage, Bytes};

use std::error::Error;
use std::io::Read;
use std::net::TcpListener;

use log::info;
use state::State;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger(7640)?;

    let config = common::config::config().expect("should get config");

    let address = format!("0.0.0.0:{}", config.runtime().port());
    let listener = TcpListener::bind(address).expect("address should be valid");
    info!("Started Listening on {}", listener.local_addr()?);

    let (alrm_signal_sender, alrm_signal_receiver) = crossbeam::channel::unbounded();
    processes::handle_alrm_signal(alrm_signal_sender);

    for backend in listener.incoming() {
        let backend = backend?;

        let peer = backend.peer_addr()?;
        info!("Connection established with {peer}");

        let mut state = State::new(backend.try_clone()?, alrm_signal_receiver.clone());
        let mut buffer = Bytes::default();
        loop {
            if backend.try_clone()?.read_exact(&mut buffer).is_err() {
                break;
            };

            let msg = match BackendToRuntimeMessage::try_from(buffer) {
                Ok(m) => m,
                Err(e) => {
                    info!("Unknown message: {e:?}");
                    continue;
                }
            };

            info!("Received message: {msg:?} {buffer:?}");
            match msg {
                BackendToRuntimeMessage::Enable => state.enable(config.runtime()),
                BackendToRuntimeMessage::Disable => state.disable()?,
            }
        }

        info!("Connection with {peer} closed.");
    }

    Ok(())
}
