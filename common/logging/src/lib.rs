extern crate core;

use config::AddressPort;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ws::Sender;

const MAX_SCROLLBACK: usize = 200;

type SingleLogJsonString = String;
type MultipleLogsJsonString = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
                    if history.lock().unwrap().len() > MAX_SCROLLBACK {
                        history.lock().unwrap().remove(0);
                    }
                }
                self.log_tx.send(json_log).unwrap();
            }
        });

        // BUG: When reloading the app this thread doesn't close.
        //      Maybe the one above too, not tested yet.
        thread::spawn({
            move || {
                ws::listen(format!("0.0.0.0:{}", self.port), |frontend| {
                    handle_frontend_client(
                        frontend,
                        Arc::clone(&self.history),
                        self.log_rx.clone(),
                    );

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

fn handle_frontend_client(
    frontend: Sender,
    history: Arc<Mutex<Vec<Log>>>,
    log_rx: crossbeam::channel::Receiver<SingleLogJsonString>,
) {
    thread::spawn({
        move || {
            // Send log history first.
            if let Ok(log_history_json) = log_vec_to_json(&history.lock().unwrap()) {
                frontend.send(ws::Message::Text(log_history_json)).unwrap()
            }

            // Periodically send logs.
            let interval_backlog = Arc::new(Mutex::new(Vec::<Log>::new()));
            thread::spawn({
                let interval_backlog = Arc::clone(&interval_backlog);
                move || loop {
                    if interval_backlog.lock().unwrap().len() > 0 {
                        let json_logs = log_vec_to_json(&interval_backlog.lock().unwrap()).unwrap();
                        frontend.send(ws::Message::Text(json_logs)).unwrap();
                        *interval_backlog.lock().unwrap() = Vec::new();
                    }

                    thread::sleep(Duration::from_millis(100));
                }
            });

            loop {
                if let Ok(json_log) = log_rx.recv() {
                    let log = json_to_log(&json_log).unwrap();
                    interval_backlog.lock().unwrap().push(log);
                }
            }
        }
    });
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
