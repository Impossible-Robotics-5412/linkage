use common::logging::setup_logger;

use crate::cockpit::start_cockpit_listener;
use crate::state::RobotStateHandle;
use crate::subsystem::Subsystem;

#[derive(Default)]
pub struct Robot {
    subsystems: Vec<Box<dyn Subsystem>>,
    setup_handler: Option<Box<dyn Fn()>>,
    tick_handler: Option<Box<dyn Fn()>>,
    shutdown_handler: Option<Box<dyn Fn()>>,
    is_running: bool,
}

impl Robot {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_subsystem<S: Subsystem + 'static>(mut self, subsystem: S) -> Self {
        self.subsystems.push(Box::new(subsystem));
        self
    }

    pub fn on_setup<F: Fn() + 'static>(mut self, on_setup: F) -> Self {
        self.setup_handler = Some(Box::new(on_setup));
        self
    }

    pub fn on_tick<F: Fn() + 'static>(mut self, on_tick: F) -> Self {
        self.tick_handler = Some(Box::new(on_tick));
        self
    }

    pub fn on_shutdown<F: Fn() + 'static>(mut self, on_shutdown: F) -> Self {
        self.shutdown_handler = Some(Box::new(on_shutdown));
        self
    }

    pub fn run(mut self) {
        setup_logger(7640).expect("logger should be able to start");

        let state: RobotStateHandle = Default::default();

        start_cockpit_listener(state.clone())
            .expect("failed to start listening for Cockpit connections.");

        self.is_running = true;
        let (term_tx, term_rx) = std::sync::mpsc::channel();

        ctrlc::set_handler(move || {
            term_tx
                .send(())
                .expect("could not send termination signal over channel")
        })
        .expect("failed to set termination handler");

        if let Some(setup) = &self.setup_handler {
            setup();
        };

        for subsystem in self.subsystems.iter_mut() {
            subsystem.setup(state.clone());
        }

        while self.is_running {
            if let Some(tick) = &self.tick_handler {
                tick();
            }
            for subsystem in self.subsystems.iter_mut() {
                subsystem.tick(state.clone());
            }

            std::thread::sleep(std::time::Duration::from_millis(20));

            if let Ok(()) = term_rx.try_recv() {
                self.is_running = false;
            }
        }

        for subsystem in self.subsystems.iter_mut() {
            subsystem.shutdown(state.clone());
        }
        if let Some(shutdown) = &self.shutdown_handler {
            shutdown();
        }
    }

    pub fn shutdown(&mut self) {
        self.is_running = false;
    }
}