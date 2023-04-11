use std::sync::{Arc, Mutex};

pub type RobotStateHandle = Arc<Mutex<RobotState>>;

#[derive(Default, Debug)]
pub struct RobotState {}
