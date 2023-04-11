use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::gamepad::{Gamepad, GamepadId};

pub type RobotStateHandle = Arc<Mutex<RobotState>>;

#[derive(Default, Debug)]
pub struct RobotState {
    pub gamepads: HashMap<GamepadId, Gamepad>,
}
