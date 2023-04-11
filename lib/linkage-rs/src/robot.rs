use common::logging::setup_logger;

use crate::subsystem::Subsystem;

#[derive(Default)]
pub struct Robot {
    subsystems: Vec<Box<dyn Subsystem>>,
    setup: Option<Box<dyn Fn()>>,
    tick: Option<Box<dyn Fn()>>,
    shutdown: Option<Box<dyn Fn()>>,
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
        self.setup = Some(Box::new(on_setup));
        self
    }

    pub fn on_tick<F: Fn() + 'static>(mut self, on_tick: F) -> Self {
        self.tick = Some(Box::new(on_tick));
        self
    }

    pub fn on_shutdown<F: Fn() + 'static>(mut self, on_shutdown: F) -> Self {
        self.shutdown = Some(Box::new(on_shutdown));
        self
    }

    pub fn run(mut self) {
        setup_logger(7682).expect("logger should be able to start");

        self.is_running = true;

        let (term_tx, term_rx) = std::sync::mpsc::channel();

        ctrlc::set_handler(move || {
            term_tx
                .send(())
                .expect("could not send termination signal over channel")
        })
        .expect("failed to set termination handler");

        if let Some(setup) = &self.setup {
            setup();
        };

        for subsystem in self.subsystems.iter_mut() {
            subsystem.setup();
        }

        while self.is_running {
            if let Some(tick) = &self.tick {
                tick();
                for subsystem in self.subsystems.iter_mut() {
                    subsystem.tick();
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(20));

            if let Ok(()) = term_rx.try_recv() {
                self.is_running = false;
            }
        }

        for subsystem in self.subsystems.iter_mut() {
            subsystem.shutdown();
        }
        if let Some(shutdown) = &self.shutdown {
            shutdown();
        }
    }

    pub fn shutdown(&mut self) {
        self.is_running = false;
    }
}
