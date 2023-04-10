use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

use common::system_info::SystemInfo;
use crossbeam::channel::Receiver;
use systemstat::Platform;

fn main() {
    let config = common::config::config().unwrap();
    let system = systemstat::System::new();
    let (tx, rx) = crossbeam::channel::unbounded();
    let client_count = Arc::new(AtomicUsize::new(0));

    // Continuously the updated system information over the channel.
    thread::spawn({
        let client_count = client_count.clone();

        move || loop {
            let system_info = SystemInfo::new(&system);

            if client_count.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                continue;
            }

            tx.send(system_info).unwrap();
        }
    });

    // Start listening for clients (Cockpit).
    let port = config.gauge().port();
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    for stream in listener.incoming() {
        let rx = rx.clone();
        let stream = stream.unwrap();
        let client_count = client_count.clone();

        stream.set_nonblocking(true).unwrap();
        client_count.fetch_add(1, Ordering::Relaxed);

        // For each client start a new thread that will handle sending the updated system information over the stream.
        thread::spawn(move || {
            handle_client(rx, stream);
            client_count.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        });
    }
}

fn handle_client(receiver: Receiver<SystemInfo>, mut stream: TcpStream) {
    // When we get updated system information from the channel:
    while let Ok(system_info) = receiver.recv() {
        // Jsonify the system info.
        let json_string = serde_json::to_string(&system_info).unwrap();
        let mut data = Vec::from(json_string);
        // Add a newline to the byte array. This indicates the end of the message.
        data.push(b'\n');

        // Send the system info over the stream to the client (Cockpit).
        _ = stream.write(&data);

        // If the stream is closed, stop this while loop.
        if let Err(error) = stream.read_exact(&mut [0]) {
            if error.kind() == ErrorKind::UnexpectedEof {
                break;
            }
        }
    }
}
