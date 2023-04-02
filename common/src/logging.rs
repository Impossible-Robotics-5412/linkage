use crate::config::AddressPort;

use std::error::Error;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use bus::Bus;
use serde::Serialize;

#[derive(Clone, Serialize)]
struct Log<'a> {
    msg: String,
    level: u8,
    file: Option<&'a str>,
    line: Option<u32>,
}

pub fn setup_logger(port: AddressPort) -> Result<(), Box<dyn Error>> {
    let (log_tx, log_rx) = channel::<String>();
    let log_bus = Arc::new(Mutex::new(Bus::<String>::new(1024)));

    setup_fern(log_tx);

    let broadcaster_log_bus = log_bus.clone();
    thread::spawn(move || loop {
        let json_log = log_rx.recv().unwrap();
        broadcaster_log_bus.lock().unwrap().broadcast(json_log);
    });

    start_websocket_server(port, log_bus);

    Ok(())
}

fn setup_fern(sender: Sender<String>) {
    fern::Dispatch::new()
        .chain(
            fern::Dispatch::new()
                .level(log::LevelFilter::Debug)
                .format(|out, message, record| {
                    out.finish(format_args!("[{}] {}", record.level(), message));
                })
                .chain(std::io::stdout()),
        )
        .chain(
            fern::Dispatch::new()
                .level(log::LevelFilter::Debug)
                .format(|out, message, record| {
                    let log = Log {
                        msg: message.to_string(),
                        level: record.level() as u8,
                        file: record.file(),
                        line: record.line(),
                    };
                    let json_log = serde_json::to_string(&log).unwrap();
                    out.finish(format_args!("{json_log}"));
                })
                .chain(sender),
        )
        .apply()
        .expect("should connect fern to sender");
}

fn start_websocket_server(port: AddressPort, log_bus: Arc<Mutex<Bus<String>>>) {
    thread::spawn(move || {
        ws::listen(format!("0.0.0.0:{port}"), |frontend| {
            let log_bus = log_bus.clone();
            thread::spawn({
                let mut log_bus_rx = log_bus.lock().unwrap().add_rx();

                move || loop {
                    match log_bus_rx.recv() {
                        Ok(json_log) => frontend.send(ws::Message::Text(json_log)).unwrap(),
                        Err(_) => {}
                    };
                }
            });

            |_msg| Ok(())
        })
        .unwrap();
    });
}
