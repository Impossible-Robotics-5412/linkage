use linkage_rs::gamepads::PsController;
use linkage_rs::motors::SparkMotorController;
use linkage_rs::prelude::*;

#[derive(Default)]
struct TankDrivetrainSubsystem;

impl Subsystem for TankDrivetrainSubsystem {
    fn tick(&mut self, state: RobotStateHandle) {
        let left_motor = SparkMotorController::new(state.clone(), 0);
        let right_motor = SparkMotorController::new(state.clone(), 1);

        let gamepad = state
            .lock()
            .unwrap()
            .gamepad_manager
            .get::<PsController>(AssociatedGamepad::Primary);

        if let Some(gamepad) = gamepad {
            left_motor.set_speed_percentage(gamepad.left_joystick_y());
            right_motor.set_speed_percentage(gamepad.right_joystick_y());
        }
    }
}

fn main() {
    Robot::new()
        .add_subsystem(TankDrivetrainSubsystem::default())
        .run();
}
