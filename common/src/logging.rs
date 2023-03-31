use crate::config::AddressPort;

use std::error::Error;
use std::sync::mpsc::channel;
use std::thread;

pub fn setup_logger(port: AddressPort) -> Result<(), Box<dyn Error>> {
    let (log_tx, log_rx) = channel();
    let (unbound_log_tx, unbound_log_rx) = crossbeam::channel::unbounded();

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
        .chain(log_tx)
        .apply()
        .expect("should connect fern to sender");

    thread::spawn(move || loop {
        let msg = log_rx.recv();
        unbound_log_tx.send(msg).unwrap();
    });

    thread::spawn(move || {
        ws::listen(format!("0.0.0.0:{port}"), |frontend| {
            let unbound_log_rx = unbound_log_rx.clone();

            thread::spawn(move || {
                while let Ok(msg) = unbound_log_rx.recv() {
                    if let Ok(msg) = msg {
                        let msg = msg.strip_suffix('\n').unwrap_or(msg.as_str());
                        frontend.send(msg).unwrap();
                    }
                }
            });

            |_msg| Ok(())
        })
        .unwrap();
    });

    Ok(())
}
