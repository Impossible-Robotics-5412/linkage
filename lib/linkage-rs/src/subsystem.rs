use crate::state::RobotStateHandle;

pub trait Subsystem {
    fn setup(&mut self, _robot_state: RobotStateHandle) {}
    fn tick(&mut self, _robot_state: RobotStateHandle) {}
    fn shutdown(&mut self, _robot_state: RobotStateHandle) {}
}
