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

struct Enabled(Vec<Child>);
struct Disabled;

enum State {
    Enabled(Enabled),
    Disabled,
}

fn state() -> State {
    State::Disabled
}

impl State {
    fn enable(&mut self, entrypoint: &str, stream: TcpStream, instruction: RuntimeInstruction) {
        eprintln!("Enabling... ");
        match self {
            State::Disabled => {
                let children = start_processes(entrypoint, stream, instruction);
                *self = Self::Enabled(Enabled(children));
                eprintln!("Enabled.");
            }
            _ => eprintln!("Already enabled, doing nothing."),
        }
    }
    fn disable(&mut self) {
        eprint!("Disabling... ");
        match self {
            State::Enabled(en) => {
                stop_processes(&mut en.0);
                *self = Self::Disabled;
                eprintln!("disabled.")
            }
            _ => eprintln!("already disabled, doing nothing."),
        }
    }
}
fn main() -> io::Result<()> {
    let settings = settings().unwrap();
    let address = format!("0.0.0.0:{}", settings.port());
    let listener = TcpListener::bind(address).expect("address should be valid");

    eprintln!("Started Listening on {}", listener.local_addr()?);

    for (n, stream) in listener.incoming().enumerate() {
        let mut stream = stream?;

        // let mut state = state();

        let peer = stream.peer_addr()?;
        eprintln!("({n}) Connection established with {peer}");

        let mut buffer = MessageBytes::default();
        loop {
            if stream.read_exact(&mut buffer).is_err() {
                break;
            };

            eprintln!("Received message: {buffer:?}");
            // match buffer[0] {
            // 0x00 => state.enable(
            //     &settings.entrypoint(),
            //     stream.try_clone()?,
            //     buffer.try_into()?,
            // ),
            // 0x01 => state.disable(),
            //     _ => eprintln!("Unknown message: {buffer:?}"),
            // }
        }

        eprintln!("({n}) Connection closed.");
    }

    Ok(())
}

fn start_processes(
    entrypoint: &str,
    stream: TcpStream,
    instruction: RuntimeInstruction,
) -> Vec<Child> {
    eprintln!("Starting Linkage");

    simple_signal::set_handler(&[Signal::Alrm], {
        let stream = Mutex::new(stream);
        move |signals| {
            eprintln!("Caught: {signals:?}");
            let msg: MessageBytes = instruction.into();
            stream.lock().unwrap().write(&msg).unwrap();
        }
    });

    let carburetor_process = Command::new("carburetor")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to execute carburetor");

    let linkage_process = Command::new("node")
        .current_dir("/")
        .arg(entrypoint)
        .spawn()
        .expect("failed to execute linkage");

    vec![carburetor_process, linkage_process]
}

fn stop_processes(children: &mut Vec<Child>) {
    for child in children {
        // TODO: deal with result
        child.kill().unwrap();
    }
}
