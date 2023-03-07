use std::error::Error;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::process::exit;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

#[cfg(target_arch = "armv7")]
use rppal::pwm::Channel;
use simple_signal::{self, Signal};

use crate::control_channel::control_channel;
use crate::instruction::{decode, Instruction, MessageBytes, Speed};

mod control_channel;
mod instruction;

#[cfg(not(target_arch = "armv7"))]
use control_channel::Channel;

const WELCOME_MESSAGE: &str = r#"
                   _
                  | |                        _
  ____ _____  ____| |__   ___   ____ _____ _| |_ ___   ____
 / ___|____ |/ ___)  _ \ / _ \ / ___|____ (_   _) _ \ / ___)
( (___/ ___ | |   | |_) ) |_| | |   / ___ | | || |_| | |
 \____)_____|_|   |____/ \___/|_|   \_____|  \__)___/|_|

             By Koen & Bauke Westendorp, 2023.
"#;

#[allow(dead_code)]
const PERIOD_MS: u64 = 20; // 20 ms = 50 Hz
#[allow(dead_code)]
const PULSE_DELTA_US: u64 = 500;
#[allow(dead_code)]
const PULSE_NEUTRAL_US: u64 = 1500;

fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("{WELCOME_MESSAGE}");

    let config = common::config::config()?;
    let address = format!("0.0.0.0:{}", config.carburetor().port());

    eprintln!("Setting up...");
    let (tx0, rx0) = channel();
    let (tx1, rx1) = channel();

    simple_signal::set_handler(&[Signal::Int, Signal::Term], {
        let tx0 = tx0.clone();
        let tx1 = tx1.clone();
        move |signals| {
            eprintln!("Caught: {signals:?}");

            // Clean up by putting both at neutral.
            eprintln!("Cleaning up...");
            tx0.send(Speed::neutral()).unwrap();
            tx1.send(Speed::neutral()).unwrap();

            // Here, we wait for 10 ms in order to give the motor control threads a chance to reset
            // the pwm to neutral. Otherwise, we might exit _before_ the neutral instruction has
            // been carried out.
            thread::sleep(Duration::from_millis(10));

            eprintln!("Bye!");
            exit(0)
        }
    });

    eprintln!("Spawning device control threads...");
    thread::spawn(|| control_channel(Channel::Pwm0, rx0));
    thread::spawn(|| control_channel(Channel::Pwm1, rx1));

    eprintln!("Setup completed. Listening on {}...", address);
    let server = TcpListener::bind(address).expect("address should be valid");
    for (n, stream) in server.incoming().enumerate() {
        let mut stream = stream?;
        let peer = stream.peer_addr()?;
        let local = stream.local_addr()?;
        eprintln!("({n}) Received stream from {peer} on {local}.",);

        let mut buf = MessageBytes::default();
        loop {
            // We read from this stream until the end of this connection into buf.
            match stream.read_exact(&mut buf) {
                Ok(_) => {}
                // If the connection was closed, break the loop.
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                // On any other error, propagate the error.
                Err(e) => return Err(e)?,
            }

            eprintln!("Received the following message:");
            eprintln!("|\t{buf:?}");

            // Decode the received json into a Vec of instructions.
            let instruction = match decode(buf) {
                Some(instr) => instr,
                None => {
                    eprintln!("|\tThis message was invalid.");
                    writeln!(stream, "Invalid message: {buf:?}.")?;
                    continue;
                }
            };

            match instruction {
                Instruction::Motor(instr) => {
                    let sender = match instr.channel() {
                        0 => tx0.clone(),
                        1 => tx1.clone(),
                        channel => {
                            eprintln!("|\tInstruction channel {channel} does not exist.");
                            continue;
                        }
                    };

                    sender.send(instr.speed())?;
                }
                // Instruction::Query => stream.write_all(&69_f32.to_be_bytes())?,
                Instruction::Query => writeln!(stream, "Nice!")?,
                Instruction::Battery => writeln!(stream, "Battery: 42%")?,
                Instruction::Memory => {
                    let info = sys_info::mem_info()?;
                    let total = info.total;
                    let free = info.free;
                    let used = total - free;
                    let percentage = (used as f32 / total as f32) * 100.0;
                    eprintln!("Reported memory status to {peer}.");
                    writeln!(stream, "Memory: {percentage:.0}% ({used} / {total})")?
                }
                Instruction::Cpu => {
                    let load = sys_info::loadavg()?.one;
                    eprintln!("Reported cpu status to {peer}.");
                    writeln!(stream, "Cpu: {load}")?
                }
            }
        }

        // Clean up by putting both at neutral.
        eprintln!("({n}) Connection closed. Resetting motors to neutral.");
        tx0.send(Speed::neutral()).unwrap();
        tx1.send(Speed::neutral()).unwrap();
        eprintln!("Still listening...");
    }

    unreachable!("server.incoming() cannot return None")
}
