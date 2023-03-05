use std::io::{self, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use simple_signal::Signal;

use crate::settings::settings;

mod settings;

type MessageBytes = [u8; 8];

#[derive(Debug, Clone, Copy)]
pub(crate) enum RuntimeInstruction {
    Enable,
    Disable,
}

impl Into<MessageBytes> for RuntimeInstruction {
    fn into(self) -> MessageBytes {
        match self {
            RuntimeInstruction::Enable => [0x00, 0, 0, 0, 0, 0, 0, 0],
            RuntimeInstruction::Disable => [0x01, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}

impl TryFrom<MessageBytes> for RuntimeInstruction {
    type Error = io::Error;

    fn try_from(value: MessageBytes) -> Result<RuntimeInstruction, io::Error> {
        match value[0] {
            0x00 => Ok(RuntimeInstruction::Enable),
            0x01 => Ok(RuntimeInstruction::Disable),
            _ => Err(io::Error::new(
                ErrorKind::Other,
                "Failed to get instruction from byte {value:x}",
            )),
        }
    }
}

enum LinkageState {
    Enabled(Vec<Child>),
    Disabled,
}

struct State {
    backend_stream: TcpStream,
    state: LinkageState,
}

impl State {
    fn new(backend_stream: TcpStream) -> Self {
        Self {
            backend_stream,
            state: LinkageState::Disabled,
        }
    }
}

impl State {
    fn enable(&mut self, entrypoint: &str) {
        eprint!("Enabling... ");
        match self.state {
            LinkageState::Disabled => {
                let children =
                    start_processes(entrypoint, self.backend_stream.try_clone().unwrap());
                self.state = LinkageState::Enabled(children);
                eprintln!("enabled.");
            }
            _ => {
                eprintln!("Already enabled, doing nothing.");
                let message_bytes: MessageBytes = RuntimeInstruction::Enable.into();
                let mut backend_stream = self.backend_stream.try_clone().unwrap();
                backend_stream.write(&message_bytes).unwrap();
            }
        }
    }

    fn disable(&mut self) -> io::Result<()> {
        eprint!("Disabling... ");
        match &mut self.state {
            LinkageState::Enabled(children) => {
                stop_processes(children)?;
                self.state = LinkageState::Disabled;
                eprintln!("disabled.");
            }
            _ => {
                eprintln!("Already disabled, doing nothing.");
                let message_bytes: MessageBytes = RuntimeInstruction::Disable.into();
                let mut backend_stream = self.backend_stream.try_clone().unwrap();
                backend_stream.write(&message_bytes).unwrap();
            }
        }

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
    let settings = settings().unwrap();
    let address = format!("0.0.0.0:{}", settings.port());
    let listener = TcpListener::bind(address).expect("address should be valid");

    eprintln!("Started Listening on {}", listener.local_addr()?);

    for (n, backend_stream) in listener.incoming().enumerate() {
        let mut backend_stream = backend_stream?;
        let mut state = State::new(backend_stream.try_clone()?);

        let peer = backend_stream.peer_addr()?;
        eprintln!("({n}) Connection established with {peer}");

        let mut buffer = MessageBytes::default();

        loop {
            if backend_stream.read_exact(&mut buffer).is_err() {
                break;
            };

            eprintln!("Received message: {buffer:?}");
            match buffer[0] {
                0x00 => state.enable(&settings.entrypoint()),
                0x01 => {
                    state.disable()?;
                    let disable_bytes: MessageBytes = RuntimeInstruction::Disable.into();
                    backend_stream
                        .write(&disable_bytes)
                        .expect("should write disable confirmation message to cockpit-backend");
                }
                _ => eprintln!("Unknown message: {buffer:?}"),
            }
        }

        eprintln!("({n}) Connection closed.");
        state.disable()?;
    }

    Ok(())
}

fn start_processes(entrypoint: &str, backend_stream: TcpStream) -> Vec<Child> {
    eprintln!("Starting Linkage");

    simple_signal::set_handler(&[Signal::Alrm], {
        let backend_stream = Mutex::new(backend_stream);
        move |signals| {
            eprintln!("Caught Signal: {signals:?}");
            let msg: MessageBytes = RuntimeInstruction::Enable.into();
            backend_stream
                .lock()
                .unwrap()
                .write(&msg)
                .expect("should write enable confirmation message to cockpit-backend");
        }
    });

    let carburetor_process = Command::new("/usr/bin/carburator")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute carburetor");

    let linkage_process = Command::new("/usr/bin/node")
        .current_dir("/")
        .arg(entrypoint)
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
