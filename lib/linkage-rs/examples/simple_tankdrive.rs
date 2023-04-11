use linkage_rs::robot::Robot;
use linkage_rs::state::RobotStateHandle;
use linkage_rs::subsystem::Subsystem;

#[derive(Default)]
struct TankDrivetrainSubsystem {
    count: usize,
}

impl Subsystem for TankDrivetrainSubsystem {
    fn tick(&mut self, robot_state: RobotStateHandle) {
        log::info!("TankDrivetrainSubsystem Tick {}", self.count);
        log::info!("{:?}", robot_state.lock().unwrap());
        self.count += 1;
    }
}

fn main() {
    Robot::new()
        .add_subsystem(TankDrivetrainSubsystem::default())
        .on_setup(|| log::info!("Robot Setup"))
        .on_tick(|| log::info!("Robot Tick"))
        .on_shutdown(|| log::info!("Robot Shutdown"))
        .run();
}
