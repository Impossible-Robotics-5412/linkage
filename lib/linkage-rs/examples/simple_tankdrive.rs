use linkage_rs::robot::Robot;
use linkage_rs::state::RobotStateHandle;
use linkage_rs::subsystem::Subsystem;

#[derive(Default)]
struct TankDrivetrainSubsystem {}

impl Subsystem for TankDrivetrainSubsystem {
    fn tick(&mut self, robot_state: RobotStateHandle) {
        log::info!("{:?}", robot_state.lock().unwrap());
    }
}

fn main() {
    Robot::new()
        .add_subsystem(TankDrivetrainSubsystem::default())
        .run();
}
