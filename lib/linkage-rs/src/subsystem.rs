use crate::state::RobotStateHandle;

pub trait Subsystem {
    fn setup(&mut self, _state: RobotStateHandle) {}
    fn tick(&mut self, _state: RobotStateHandle) {}
    fn shutdown(&mut self, _state: RobotStateHandle) {}
}
