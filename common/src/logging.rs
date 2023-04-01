use crate::config::AddressPort;

use std::error::Error;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use bus::Bus;

pub fn setup_logger(port: AddressPort) -> Result<(), Box<dyn Error>> {
    let (log_tx, log_rx) = channel();
    let log_bus = Arc::new(Mutex::new(Bus::new(1024)));

    setup_fern(log_tx);

    let broadcaster_log_bus = log_bus.clone();
    thread::spawn(move || loop {
        let msg = log_rx.recv().unwrap();
        broadcaster_log_bus.lock().unwrap().broadcast(msg);
    });

    start_websocket_server(port, log_bus);

    Ok(())
}

fn setup_fern(sender: Sender<String>) {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ));
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(sender)
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
                        Ok(msg) => {
                            let msg = msg.strip_suffix('\n').unwrap_or(msg.as_str());
                            frontend.send(msg).unwrap();
                        }
                        Err(_) => {
                            frontend.send(format!("CLOSED")).unwrap();
                        }
                    };
                }
            });

            |_msg| Ok(())
        })
        .unwrap();
    });
}
