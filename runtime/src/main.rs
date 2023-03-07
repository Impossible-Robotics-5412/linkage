use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};

use common::config;
use simple_signal::Signal;

type MessageBytes = [u8; 8];

const RUNTIME_TX_ENABLED: MessageBytes = [0x00, 0, 0, 0, 0, 0, 0, 0];
const RUNTIME_RX_DISABLED: MessageBytes = [0x01, 0, 0, 0, 0, 0, 0, 0];

enum LinkageState {
    Enabled(Vec<Child>),
    Disabled,
}

struct State {
    backend: TcpStream,
    state: LinkageState,
    alrm_signal_receiver: Receiver<()>,
}

impl State {
    fn new(backend_stream: TcpStream) -> Self {
        let (alrm_signal_sender, alrm_signal_receiver) = std::sync::mpsc::channel();
        handle_alrm_signal(alrm_signal_sender);

        Self {
            backend: backend_stream,
            state: LinkageState::Disabled,
            alrm_signal_receiver,
        }
    }
}

impl State {
    fn enable(&mut self, config: &config::Runtime) {
        eprintln!("Enabling Linakge... ");
        match self.state {
            LinkageState::Disabled => {
                let children = start_processes(config);

                self.alrm_signal_receiver.recv().unwrap();

                self.state = LinkageState::Enabled(children);
                eprintln!("Linkage Enabled.");
            }
            _ => eprintln!("Already enabled, doing nothing."),
        }

        self.backend.write(&RUNTIME_TX_ENABLED).unwrap();
    }

    fn disable(&mut self) -> io::Result<()> {
        eprintln!("Disabling Linkage... ");
        match &mut self.state {
            LinkageState::Enabled(children) => {
                stop_processes(children)?;
                self.state = LinkageState::Disabled;
                eprintln!("Linkage Disabled.");
            }
            _ => eprintln!("Already disabled, doing nothing."),
        }

        self.backend.write(&RUNTIME_RX_DISABLED)?;

        Ok(())
    }
}

impl Drop for State {
    fn drop(&mut self) {
        self.disable()
            .expect("should shut down child processes on drop");
    }
}

fn main() -> io::Result<()> {
    let config = common::config::config().expect("should get config");
    let address = format!("0.0.0.0:{}", config.runtime().port());
    let listener = TcpListener::bind(address).expect("address should be valid");
    eprintln!("Started Listening on {}", listener.local_addr()?);

    let (backend, _) = listener.accept()?;
    let peer = backend.peer_addr()?;
    eprintln!("Connection established with {peer}");

    let mut state = State::new(backend.try_clone()?);

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

    eprintln!("Connection closed.");
    state.disable()?;

    Ok(())
}

fn handle_alrm_signal(sender: Sender<()>) {
    simple_signal::set_handler(&[Signal::Alrm], {
        move |signals| {
            eprintln!("Caught Signal: {signals:?}");
            sender.send(()).expect("should be a valid channel");
        }
    });
}

fn start_processes(config: &config::Runtime) -> Vec<Child> {
    eprintln!("Starting Linkage");

    let carburetor_process = Command::new(config.carburetor_path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute carburetor");

    let linkage_process = Command::new(config.node_path())
        .current_dir("/")
        .arg(config.linkage_lib_entry_point())
        .spawn()
        .expect("failed to execute linkage");

    vec![carburetor_process, linkage_process]
}

fn stop_processes(children: &mut Vec<Child>) -> io::Result<()> {
    for child in children {
        child.kill()?;
        child.wait()?;
    }

    Ok(())
}
