use crate::processes;

use common::config;
use common::messages::{Message, RuntimeToBackendMessage};

use std::io::{self, Write};
use std::net::TcpStream;
use std::process::Child;

use log::info;

pub(crate) enum LinkageState {
    Enabled(Vec<Child>),
    Disabled,
}

pub(crate) struct State {
    backend: TcpStream,
    state: LinkageState,
    alrm_signal_receiver: crossbeam::channel::Receiver<()>,
}

impl State {
    pub(crate) fn new(
        backend_stream: TcpStream,
        alrm_signal_receiver: crossbeam::channel::Receiver<()>,
    ) -> Self {
        Self {
            backend: backend_stream,
            state: LinkageState::Disabled,
            alrm_signal_receiver,
        }
    }
}

impl State {
    pub(crate) fn enable(&mut self, config: &config::Runtime) {
        info!("Enabling Linakge... ");
        match self.state {
            LinkageState::Disabled => {
                let children = processes::start_processes(config);

                self.alrm_signal_receiver
                    .recv()
                    .expect("should receive alrm signal");

                self.state = LinkageState::Enabled(children);
                info!("Linkage Enabled.");
            }
            _ => info!("Already enabled, doing nothing."),
        }

        self.backend
            .write(&RuntimeToBackendMessage::Enabled.to_bytes())
            .expect("should send the ENABLED message to cockpit-backend");
    }

    pub(crate) fn disable(&mut self) -> io::Result<()> {
        info!("Disabling Linkage... ");
        match &mut self.state {
            LinkageState::Enabled(children) => {
                processes::stop_processes(children)?;
                self.state = LinkageState::Disabled;
                info!("Linkage Disabled.");
            }
            _ => info!("Already disabled, doing nothing."),
        }

        self.backend
            .write(&RuntimeToBackendMessage::Disabled.to_bytes())
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
