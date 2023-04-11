use std::{thread, time::Duration};

pub struct Robot {}

impl Robot {
    pub fn new() -> Self {
        Self {}
    }
}

impl Robot {
    pub fn run(self) {
        let event_loop_handle = thread::spawn(move || loop {
            eprintln!("Running");
            thread::sleep(Duration::from_secs(1));
        });

        event_loop_handle.join().unwrap();
    }
}
