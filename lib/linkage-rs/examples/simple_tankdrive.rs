use linkage_rs::gamepads::gamepad::Gamepad;
use linkage_rs::gamepads::ps_controller::PsController;
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
        log::info!(
            "Triangle pressed on PS Controller: {}",
            ps_controller.triangle()
        );
    }
}

fn main() {
    Robot::new()
        .add_subsystem(TankDrivetrainSubsystem::default())
        .run();
}
