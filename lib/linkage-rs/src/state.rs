use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use common::messages::LinkageToCarburetor;

use crate::gamepads::gamepad::{GamepadData, GamepadId};

pub type RobotStateHandle = Arc<Mutex<RobotState>>;

#[derive(Debug)]
pub struct RobotState {
    pub gamepads: HashMap<GamepadId, GamepadData>,

    pub(crate) carburetor_message_sender: Sender<LinkageToCarburetor>,
}

impl RobotState {
    pub fn new(carburetor_message_sender: Sender<LinkageToCarburetor>) -> Self {
        Self {
            gamepads: HashMap::new(),
            carburetor_message_sender,
        }
    }
}
