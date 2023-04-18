use config::AddressPort;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize)]
struct Log {
    msg: String,
    level: u8,
    file: Option<String>,
    line: Option<u32>,
    timestamp: u128,
}

pub struct Logger {
    port: AddressPort,
    history: Arc<Mutex<Vec<String>>>,
    fern_tx: std::sync::mpsc::Sender<String>,
    fern_rx: std::sync::mpsc::Receiver<String>,
    log_tx: crossbeam::channel::Sender<String>,
    log_rx: crossbeam::channel::Receiver<String>,
}

impl Logger {
    pub fn new(port: AddressPort) -> Self {
        let (fern_tx, fern_rx) = std::sync::mpsc::channel::<String>();
        let (log_tx, log_rx) = crossbeam::channel::unbounded::<String>();
        Self {
            port,
            fern_tx,
            fern_rx,
            log_tx,
            log_rx,
            history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(self) {
        self.setup_fern(self.fern_tx.clone());

        thread::spawn({
            let history = self.history.clone();
            move || loop {
                let json_log = self.fern_rx.recv().unwrap();
                history.lock().unwrap().push(json_log.clone());
                self.log_tx.send(json_log).unwrap();
            }
        });

        thread::spawn({
            let history = self.history.clone();
            move || {
                ws::listen(format!("0.0.0.0:{}", self.port), |frontend| {
                    let log_rx = self.log_rx.clone();

                    for json_log in history.lock().unwrap().as_slice() {
                        frontend
                            .send(ws::Message::Text(json_log.to_owned()))
                            .unwrap()
                    }

                    thread::spawn({
                        move || loop {
                            if let Ok(json_log) = log_rx.recv() {
                                frontend.send(ws::Message::Text(json_log)).unwrap()
                            }
                        }
                    });

                    |_msg| Ok(())
                })
                .unwrap();
            }
        });

        log::info!("Logger started on port {}", self.port);
    }

    fn setup_fern(&self, fern_tx: std::sync::mpsc::Sender<String>) {
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
                            file: record.file().map(|f| f.to_string()),
                            line: record.line(),
                            timestamp: SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("SystemTime before UNIX EPOCH!")
                                .as_millis(),
                        };
                        let json_log = serde_json::to_string(&log).unwrap();
                        out.finish(format_args!("{json_log}"));
                    })
                    .chain(fern_tx),
            )
            .apply()
            .expect("should connect fern to sender");
    }
}
