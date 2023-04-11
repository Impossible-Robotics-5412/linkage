use linkage_rs::robot::Robot;
use linkage_rs::subsystem::Subsystem;

#[derive(Default)]
struct TankDrivetrainSubsystem {
    count: usize,
}

impl Subsystem for TankDrivetrainSubsystem {
    fn setup(&mut self) {
        eprintln!("TankDrivetrainSubsystem Setup")
    }

    fn tick(&mut self) {
        eprintln!("TankDrivetrainSubsystem Tick {}", self.count);
        self.count += 1;
    }

    fn shutdown(&mut self) {
        eprintln!("TankDrivetrainSubsystem Shutdown")
    }
}

fn main() {
    Robot::new()
        .add_subsystem(TankDrivetrainSubsystem::default())
        .on_setup(|| eprintln!("Robot Setup"))
        .on_tick(|| eprintln!("Robot Tick"))
        .on_shutdown(|| eprintln!("Robot Shutdown"))
        .run();
}
