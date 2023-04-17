use config::AddressPort;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use system_info::{encode_system_info, SystemInfo};
use systemstat::{Platform, System};

const UPDATE_INTERVAL_MILLIS: u64 = 500;

pub struct Gauge {
    port: AddressPort,
}

impl Gauge {
    pub fn new(port: AddressPort) -> Self {
        Self { port }
    }

    pub fn start(&self) {
        let (tx, rx) = crossbeam::channel::unbounded();
        let system = System::new();
        let client_count = Arc::new(AtomicUsize::new(0));

        // Continuously the updated system information over the channel.
        std::thread::spawn({
            let client_count = Arc::clone(&client_count);
            move || loop {
                let system_info =
                    SystemInfo::new(&system, Duration::from_millis(UPDATE_INTERVAL_MILLIS));

                if client_count.load(Ordering::Relaxed) == 0 {
                    continue;
                }

                tx.send(system_info).unwrap();
            }
        });

        // Start listening for clients (Cockpit).
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).unwrap();
        for stream in listener.incoming() {
            let rx = rx.clone();
            let stream = stream.unwrap();
            let client_count = Arc::clone(&client_count);

            stream.set_nonblocking(true).unwrap();
            client_count.fetch_add(1, Ordering::Relaxed);

            // For each client start a new thread that will handle sending the updated system information over the stream.
            std::thread::spawn(move || {
                handle_client(rx, stream);
                client_count.fetch_sub(1, Ordering::Relaxed);
            });
        }
    }
}

fn handle_client(receiver: crossbeam::channel::Receiver<SystemInfo>, mut stream: TcpStream) {
    // When we get updated system information from the channel:
    while let Ok(system_info) = receiver.recv() {
        // Send the system info over the stream to the client (Cockpit).
        _ = stream.write(&encode_system_info(&system_info).as_bytes());

        // If the stream is closed, stop this while loop.
        if let Err(error) = stream.read_exact(&mut [0]) {
            if error.kind() == ErrorKind::UnexpectedEof {
                break;
            }
        }
    }
}
