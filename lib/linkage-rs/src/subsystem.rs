//! Subsystems are modular components that can perform specific tasks or control specific hardware.

use crate::state::RobotStateHandle;

/// A trait representing a robot subsystem.
///
/// Subsystems are modular components that can perform specific tasks or control specific hardware. They can be
/// added to a Robot instance and their lifecycle methods (setup, tick, and shutdown) will be called automatically.
/// Example of how to implement and use the Subsystem trait:
///
/// ```
/// use linkage_rs::gamepads::PsController;
/// use linkage_rs::motors::SparkMotorController;
/// use linkage_rs::prelude::*;
///
/// #[derive(Default)]
/// struct ExampleSubsystem;
///
/// impl Subsystem for ExampleSubsystem {
///     fn setup(&mut self, state: RobotStateHandle) {
///         log::info!("This is called once at the start of the Robot lifecycle");
///     }
///
///     fn tick(&mut self, state: RobotStateHandle) {
///         log::info!("This is called once every 20ms (50hz)");
///     }
///
///     fn shutdown(&mut self, state: RobotStateHandle) {
///         log::info!("This is called once at the end of the Robot lifecycle");
///     }
/// }
///
///  Robot::new()
///     .add_subsystem(ExampleSubsystem::default())
///     .run();
/// ```
pub trait Subsystem {
    /// Called after the [Robot][`crate::robot::Robot`] has been set up.
    /// Override this method to perform initial setup tasks.
    ///
    /// # Arguments
    ///
    /// * `state` - A handle to the shared RobotState.
    #[allow(unused_variables)]
    fn setup(&mut self, state: RobotStateHandle) {}

    /// Called periodically every 20ms (50hz) while the [Robot][`crate::robot::Robot`] is running.
    /// Override this method to implement the main logic of the subsystem.
    ///
    /// # Arguments
    ///
    /// * `state` - A handle to the shared RobotState.
    #[allow(unused_variables)]
    fn tick(&mut self, state: RobotStateHandle) {}

    /// Called before the [Robot][`crate::robot::Robot`] is shutting down.
    /// Override this method to perform cleanup tasks.
    ///
    /// # Arguments
    ///
    /// * `state` - A handle to the shared RobotState.
    #[allow(unused_variables)]
    fn shutdown(&mut self, state: RobotStateHandle) {}
}
