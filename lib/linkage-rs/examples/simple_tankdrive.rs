use linkage_rs::gamepads::gamepad_manager::GamepadIndex;
use linkage_rs::gamepads::ps_controller::PsController;
use linkage_rs::motors::spark_motor_controller::SparkMotorController;
use linkage_rs::robot::Robot;
use linkage_rs::state::RobotStateHandle;
use linkage_rs::subsystem::Subsystem;

#[derive(Default)]
struct TankDrivetrainSubsystem {}

impl Subsystem for TankDrivetrainSubsystem {
    fn tick(&mut self, state: RobotStateHandle) {
        let left_motor = SparkMotorController::new(state.clone(), 0);
        let right_motor = SparkMotorController::new(state.clone(), 1);

        let gamepad = state
            .lock()
            .unwrap()
            .gamepad_manager
            .get::<PsController>(GamepadIndex::Primary);

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
