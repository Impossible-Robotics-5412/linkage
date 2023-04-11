use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::gamepads::gamepad::{GamepadData, GamepadId};

pub type RobotStateHandle = Arc<Mutex<RobotState>>;

#[derive(Default, Debug)]
pub struct RobotState {
    pub gamepads: HashMap<GamepadId, GamepadData>,
}
