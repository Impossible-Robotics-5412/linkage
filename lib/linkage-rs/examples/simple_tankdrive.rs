use linkage_rs::gamepads::gamepad::Gamepad;
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

        let mut gamepads = state.lock().unwrap().gamepads.to_owned();
        let primary_gamepad = PsController::new(gamepads.entry(0).or_default().to_owned());

        left_motor.set_speed_percentage(primary_gamepad.left_joystick_y());
        right_motor.set_speed_percentage(primary_gamepad.right_joystick_y());
    }
}

fn main() {
    Robot::new()
        .add_subsystem(TankDrivetrainSubsystem::default())
        .run();
}
