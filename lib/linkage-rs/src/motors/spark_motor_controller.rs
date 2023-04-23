use messaging::LinkageToCarburetor;

use crate::state::RobotStateHandle;

/// Used control Spark-branded motor controllers.
///
/// This struct allows you to create motor controllers for specific channels and set the motor speed as a percentage.
pub struct SparkMotorController {
    state: RobotStateHandle,
    channel: u8,
}

impl SparkMotorController {
    /// Creates a new [`SparkMotorController`] with the specified shared [RobotState][`crate::state::RobotState`] and channel.
    ///
    /// # Arguments
    ///
    /// * `state` - A handle to the shared [RobotState][`crate::state::RobotState`].
    /// * `channel` - The channel that this motor controller should control.
    ///
    /// # Returns
    ///
    /// A new instance of [`SparkMotorController`].
    pub fn new(state: RobotStateHandle, channel: u8) -> Self {
        Self { state, channel }
    }

    /// Sets the motor speed as a percentage.
    ///
    /// # Arguments
    ///
    /// * `speed` - The desired motor speed as a percentage (between -1.0 and 1.0).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use linkage_rs::gamepads::PsController;
    /// use linkage_rs::motors::SparkMotorController;
    /// use linkage_rs::prelude::*;
    ///
    /// #[derive(Default)]
    /// struct ExampleSubsystem;
    ///
    /// impl Subsystem for ExampleSubsystem {
    ///     fn setup(&mut self, state: RobotStateHandle) {
    ///         let motor = SparkMotorController::new(state.clone(), 0); // Create a new SparkMotorController with channel 0.     
    ///         motor.set_speed_percentage(0.5); // Set motor speed to 50%.
    ///     }
    /// }
    ///
    ///  Robot::new()
    ///     .add_subsystem(ExampleSubsystem::default())
    ///     .run();
    /// ```
    pub fn set_speed_percentage(&self, speed: f32) {
        let sender = self.state.lock().unwrap().carburetor_message_sender.clone();

        sender
            .send(LinkageToCarburetor::MotorInstruction {
                channel: self.channel,
                speed,
            })
            .unwrap();
    }
}
