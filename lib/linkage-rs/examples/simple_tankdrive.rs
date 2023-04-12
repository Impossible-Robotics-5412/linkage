use linkage_rs::gamepads::gamepad::Gamepad;
use linkage_rs::gamepads::ps_controller::PsController;
use linkage_rs::motors::spark_motor_controller::SparkMotorController;
use linkage_rs::robot::Robot;
use linkage_rs::state::RobotStateHandle;
use linkage_rs::subsystem::Subsystem;

#[derive(Default)]
struct TankDrivetrainSubsystem {}

impl Subsystem for TankDrivetrainSubsystem {
    fn tick(&mut self, robot_state: RobotStateHandle) {
        let mut gamepads = robot_state.lock().unwrap().gamepads.to_owned();
        let gamepad = gamepads.entry(0).or_default();
        let ps_controller = PsController::new(gamepad.clone());

        let motor = SparkMotorController::new(robot_state.clone(), 0);
        motor.set_speed_percentage(ps_controller.left_joystick_y());

        let motor = SparkMotorController::new(robot_state, 1);
        motor.set_speed_percentage(ps_controller.right_joystick_y());
    }
}

fn main() {
    Robot::new()
        .add_subsystem(TankDrivetrainSubsystem::default())
        .run();
}
