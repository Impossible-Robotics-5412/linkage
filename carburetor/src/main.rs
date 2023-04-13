use std::error::Error;
use std::io::{ErrorKind, Read};
use std::net::TcpListener;
use std::process::exit;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use common::logging::setup_logger;

use common::messages::LinkageToCarburetor;
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
use rppal::pwm::Channel;
use simple_signal::{self, Signal};

use crate::control_channel::control_channel;
use crate::instruction::{MessageBytes, Speed};

mod control_channel;
mod instruction;

#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
use control_channel::Channel;

const WELCOME_MESSAGE: &str = r#"
                   _
                  | |                        _
  ____ _____  ____| |__  _   _  ____ _____ _| |_ ___   ____
 / ___|____ |/ ___)  _ \| | | |/ ___) ___ (_   _) _ \ / ___)
( (___/ ___ | |   | |_) ) |_| | |   | ____| | || |_| | |
 \____)_____|_|   |____/|____/|_|   |_____)  \__)___/|_|

             By Koen & Bauke Westendorp, 2023.
"#;

#[allow(dead_code)]
const PERIOD_MS: u64 = 20; // 20 ms = 50 Hz
#[allow(dead_code)]
const PULSE_DELTA_US: u64 = 500;
#[allow(dead_code)]
const PULSE_NEUTRAL_US: u64 = 1500;

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger(7644)?;

    log::info!("{WELCOME_MESSAGE}");

    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    log::info!("Carburetor detected you are running on a Raspberry Pi!");

    let config = common::config::config()?;
    let address = format!("0.0.0.0:{}", config.carburetor().port());

    log::info!("Setting up...");
    let (tx0, rx0) = channel();
    let (tx1, rx1) = channel();

    simple_signal::set_handler(&[Signal::Int, Signal::Term], {
        let tx0 = tx0.clone();
        let tx1 = tx1.clone();
        move |signals| {
            log::info!("Caught: {signals:?}");

            // Clean up by putting both at neutral.
            log::info!("Cleaning up...");
            tx0.send(Speed::neutral()).unwrap();
            tx1.send(Speed::neutral()).unwrap();

            // Here, we wait for 10 ms in order to give the motor control threads a chance to reset
            // the pwm to neutral. Otherwise, we might exit _before_ the neutral instruction has
            // been carried out.
            thread::sleep(Duration::from_millis(10));

            log::info!("Bye!");
            exit(0)
        }
    });

    log::info!("Spawning device control threads...");
    thread::spawn(|| control_channel(Channel::Pwm0, rx0));
    thread::spawn(|| control_channel(Channel::Pwm1, rx1));

    log::info!("Setup completed. Listening on {}...", address);
    let server = TcpListener::bind(address).expect("address should be valid");
    for (n, stream) in server.incoming().enumerate() {
        let mut stream = stream?;
        let peer = stream.peer_addr()?;
        let local = stream.local_addr()?;
        log::info!("({n}) Received stream from {peer} on {local}.",);

        let mut message_bytes = MessageBytes::default();
        loop {
            // We read from this stream until the end of this connection into buf.
            match stream.read_exact(&mut message_bytes) {
                Ok(_) => {}
                // If the connection was closed, break the loop.
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                // On any other error, propagate the error.
                Err(e) => return Err(e)?,
            }

            log::trace!("Received message: {message_bytes:?}");

            match LinkageToCarburetor::try_from(message_bytes) {
                Ok(message) => match message {
                    LinkageToCarburetor::MotorInstruction { channel, speed } => {
                        let sender = match channel {
                            0 => tx0.clone(),
                            1 => tx1.clone(),
                            channel => {
                                log::error!("Instruction channel {channel} does not exist.");
                                continue;
                            }
                        };

                        if let Some(speed) = Speed::new(speed) {
                            sender.send(speed)?;
                        }
                    }
                },
                Err(e) => return Err(e)?,
            }
        }

        // Clean up by putting both at neutral.
        log::info!("({n}) Connection closed. Resetting motors to neutral.");
        tx0.send(Speed::neutral()).unwrap();
        tx1.send(Speed::neutral()).unwrap();
        log::info!("Still listening...");
    }

    unreachable!("server.incoming() cannot return None")
}
