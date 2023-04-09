use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

use crossbeam::channel::Receiver;
use system::SystemInfo;
use systemstat::Platform;

extern crate systemstat;

mod system;

fn main() {
    let config = common::config::config().unwrap();

    let system = systemstat::System::new();

    let (tx, rx) = crossbeam::channel::unbounded();

    // Continuously the updated system information over the channel.
    thread::spawn(move || loop {
        let system_info = system::SystemInfo::new(&system);

        // FIXME: we should only send if we have listeners.
        // Otherwise thousands of pending messages will stack up because they won't be read.
        tx.send(system_info).unwrap();
    });

    // Start listening for clients (Cockpit).
    let port = config.gauge().port();
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    for stream in listener.incoming() {
        let rx = rx.clone();

        // for each client start a new thread that will handle sending the updated system information over the stream.
        thread::spawn(move || {
            handle_client(rx, stream.unwrap());
        });
    }
}

fn handle_client(receiver: Receiver<SystemInfo>, mut stream: TcpStream) {
    // When we get updated system information from the channel:
    while let Ok(system_info) = receiver.recv() {
        // Jsonify the system info.
        let json_string = serde_json::to_string(&system_info).unwrap();

        // Send the system info over the stream to the client (Cockpit).
        stream.write(json_string.as_bytes()).unwrap();
    }
}
