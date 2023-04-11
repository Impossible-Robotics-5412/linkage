use linkage_rs::robot::Robot;
use linkage_rs::subsystem::Subsystem;

#[derive(Default)]
struct TankDrivetrainSubsystem {
    count: usize,
}

impl Subsystem for TankDrivetrainSubsystem {
    fn setup(&mut self) {
        log::info!("TankDrivetrainSubsystem Setup")
    }

    fn tick(&mut self) {
        log::info!("TankDrivetrainSubsystem Tick {}", self.count);
        self.count += 1;
    }

    fn shutdown(&mut self) {
        log::info!("TankDrivetrainSubsystem Shutdown")
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
