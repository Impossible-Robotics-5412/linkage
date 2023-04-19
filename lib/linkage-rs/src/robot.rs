//! The entrypoint for your robot code, and encapsulates the event loop.

use std::io::{self, Read};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

use crate::carburetor;
use crate::cockpit;
use crate::state::RobotState;
use crate::subsystem::Subsystem;

/// A struct representing the main robot object.
/// Manages subsystems and handles setup, tick, and shutdown events.
///
/// # Examples
///
/// ```
/// use linkage_rs::prelude::*;
///
/// fn main() {
///     Robot::new()
///         .run();
/// }
/// ```
#[derive(Default)]
pub struct Robot {
    subsystems: Vec<Box<dyn Subsystem>>,
    setup_handler: Option<Box<dyn Fn()>>,
    tick_handler: Option<Box<dyn Fn()>>,
    shutdown_handler: Option<Box<dyn Fn()>>,
    is_running: bool,
}

impl Robot {
    /// Creates a new [`Robot`] instance with default values.
    ///
    /// # Returns
    ///
    /// * A new `Robot` instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a new subsystem to the robot.
    ///
    /// # Arguments
    ///
    /// * `subsystem` - The subsystem to add.
    ///
    /// # Returns
    ///
    /// * The `Robot` instance with the subsystem added.
    pub fn add_subsystem<S: Subsystem + 'static>(mut self, subsystem: S) -> Self {
        self.subsystems.push(Box::new(subsystem));
        self
    }

    /// Sets the setup handler function for the robot. This will be called once when the robot
    /// has been set up.
    ///
    /// # Arguments
    ///
    /// * `on_setup` - The setup handler function.
    ///
    /// # Returns
    ///
    /// * The `Robot` instance with the setup handler set.
    pub fn on_setup<F: Fn() + 'static>(mut self, setup_handler: F) -> Self {
        self.setup_handler = Some(Box::new(setup_handler));
        self
    }

    /// Sets the tick handler function for the robot. This will be called once every 20ms (50hz)
    ///
    /// # Arguments
    ///
    /// * `on_tick` - The tick handler function.
    ///
    /// # Returns
    ///
    /// * The `Robot` instance with the tick handler set.
    pub fn on_tick<F: Fn() + 'static>(mut self, tick_handler: F) -> Self {
        self.tick_handler = Some(Box::new(tick_handler));
        self
    }

    /// Sets the shutdown handler function for the robot. This will be called once all subsystems
    /// have been shut down.
    ///
    /// # Arguments
    ///
    /// * `on_shutdown` - The shutdown handler function.
    ///
    /// # Returns
    ///
    /// * The `Robot` instance with the shutdown handler set.
    pub fn on_shutdown<F: Fn() + 'static>(mut self, shutdown_handler: F) -> Self {
        self.shutdown_handler = Some(Box::new(shutdown_handler));
        self
    }

    /// Runs the main loop of the robot, executing the setup, tick, and shutdown handlers.
    pub fn run(mut self) {
        let config = config::config().expect("failed to load config");
        logging::Logger::new(config.linkage_lib().logger_port().to_owned()).start();

        let (carburetor_message_sender, carburetor_message_receiver) = channel();

        let state = Arc::new(Mutex::new(RobotState::new(carburetor_message_sender)));

        cockpit::start_listener(state.clone(), config.linkage_lib().port())
            .expect("failed to start listening for Cockpit connections.");

        carburetor::open_connection(
            carburetor_message_receiver,
            config.linkage_lib().carburetor_address(),
        )
        .expect("failed to open connection with Cockpit.");

        self.is_running = true;
        let (term_tx, term_rx) = channel();

        ctrlc::set_handler({
            let term_tx = term_tx.clone();
            move || {
                term_tx
                    .send(())
                    .expect("could not send termination signal over channel")
            }
        })
        .expect("failed to set termination handler");

        // NOTE: This makes sure we close the connection when te systemd socket
        //       is closed. It will close when the stdin stream is closed.
        std::thread::spawn(move || {
            let mut stdin = io::stdin();
            let mut buffer = [0; 1024];

            loop {
                match stdin.read(&mut buffer) {
                    Ok(n) => {
                        if n == 0 {
                            term_tx
                                .send(())
                                .expect("could not send termination signal over channel");
                            break;
                        }
                    }
                    Err(e) => {
                        log::debug!("Error reading from stdin: {}", e);
                        break;
                    }
                }
            }
        });

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

    /// Shuts down the robot, stopping its main loop.
    pub fn shutdown(&mut self) {
        self.is_running = false;
    }
}
