extern crate core;

use config::AddressPort;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

type SingleLogJsonString = String;
type MultipleLogsJsonString = String;

#[derive(Clone, Serialize, Deserialize)]
struct Log {
    msg: String,
    level: u8,
    file: Option<String>,
    line: Option<u32>,
    timestamp: u128,
}

pub struct Logger {
    port: AddressPort,
    history: Arc<Mutex<Vec<Log>>>,
    fern_tx: std::sync::mpsc::Sender<SingleLogJsonString>,
    fern_rx: std::sync::mpsc::Receiver<SingleLogJsonString>,
    log_tx: crossbeam::channel::Sender<MultipleLogsJsonString>,
    log_rx: crossbeam::channel::Receiver<MultipleLogsJsonString>,
}

impl Logger {
    pub fn new(port: AddressPort) -> Self {
        let (fern_tx, fern_rx) = std::sync::mpsc::channel::<SingleLogJsonString>();
        let (log_tx, log_rx) = crossbeam::channel::unbounded::<MultipleLogsJsonString>();
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
                if let Ok(log) = json_to_log(&json_log) {
                    history.lock().unwrap().push(log);
                }
                self.log_tx.send(json_log).unwrap();
            }
        });

        thread::spawn({
            let history = self.history.clone();
            move || {
                ws::listen(format!("0.0.0.0:{}", self.port), |frontend| {
                    let log_rx = self.log_rx.clone();
                    let log_history = history.lock().unwrap().to_vec();

                    thread::spawn({
                        move || {
                            // Send log history first.
                            if let Ok(log_history_json) = log_vec_to_json(&log_history) {
                                frontend.send(ws::Message::Text(log_history_json)).unwrap()
                            }

                            loop {
                                if let Ok(json_log) = log_rx.recv() {
                                    let log = json_to_log(&json_log).unwrap();
                                    let json_logs = log_vec_to_json(&vec![log]).unwrap();
                                    frontend.send(ws::Message::Text(json_logs)).unwrap()
                                }
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

    fn setup_fern(&self, fern_tx: std::sync::mpsc::Sender<SingleLogJsonString>) {
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
                        let json_log = log_to_json(&log).unwrap();
                        out.finish(format_args!("{json_log}"));
                    })
                    .chain(fern_tx),
            )
            .apply()
            .expect("should connect fern to sender");
    }
}

fn log_vec_to_json(logs: &Vec<Log>) -> serde_json::Result<MultipleLogsJsonString> {
    serde_json::to_string(logs.as_slice())
}

fn json_to_log(json: &SingleLogJsonString) -> serde_json::Result<Log> {
    serde_json::from_value(json.parse().unwrap())
}

fn log_to_json(log: &Log) -> serde_json::Result<SingleLogJsonString> {
    serde_json::to_string(&log)
}
