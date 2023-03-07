use crate::{processes, MessageBytes};

use common::config;
use std::{
    io::{self, Write},
    net::TcpStream,
    process::Child,
    sync::mpsc::Receiver,
};

const RUNTIME_TX_ENABLED: MessageBytes = [0x00, 0, 0, 0, 0, 0, 0, 0];
const RUNTIME_RX_DISABLED: MessageBytes = [0x01, 0, 0, 0, 0, 0, 0, 0];

pub(crate) enum LinkageState {
    Enabled(Vec<Child>),
    Disabled,
}

pub(crate) struct State {
    backend: TcpStream,
    state: LinkageState,
    alrm_signal_receiver: Receiver<()>,
}

impl State {
    pub(crate) fn new(backend_stream: TcpStream) -> Self {
        let (alrm_signal_sender, alrm_signal_receiver) = std::sync::mpsc::channel();
        processes::handle_alrm_signal(alrm_signal_sender);

        Self {
            backend: backend_stream,
            state: LinkageState::Disabled,
            alrm_signal_receiver,
        }
    }
}

impl State {
    pub(crate) fn enable(&mut self, config: &config::Runtime) {
        eprintln!("Enabling Linakge... ");
        match self.state {
            LinkageState::Disabled => {
                let children = processes::start_processes(config);

                self.alrm_signal_receiver
                    .recv()
                    .expect("should receive alrm signal");

                self.state = LinkageState::Enabled(children);
                eprintln!("Linkage Enabled.");
            }
            _ => eprintln!("Already enabled, doing nothing."),
        }

        self.backend
            .write(&RUNTIME_TX_ENABLED)
            .expect("should send the ENABLED message to cockpit-backend");
    }

    pub(crate) fn disable(&mut self) -> io::Result<()> {
        eprintln!("Disabling Linkage... ");
        match &mut self.state {
            LinkageState::Enabled(children) => {
                processes::stop_processes(children)?;
                self.state = LinkageState::Disabled;
                eprintln!("Linkage Disabled.");
            }
            _ => eprintln!("Already disabled, doing nothing."),
        }

        self.backend
            .write(&RUNTIME_RX_DISABLED)
            .expect("should send the DISABLED message to cockpit-backend");

        Ok(())
    }
}

impl Drop for State {
    fn drop(&mut self) {
        self.disable()
            .expect("should shut down child processes on drop");
    }
}
