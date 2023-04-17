use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use messaging::LinkageToCarburetor;

use crate::gamepads::gamepad_manager::GamepadManager;

pub type RobotStateHandle = Arc<Mutex<RobotState>>;

#[derive(Debug)]
pub struct RobotState {
    pub gamepad_manager: GamepadManager,
    pub(crate) carburetor_message_sender: Sender<LinkageToCarburetor>,
}

impl RobotState {
    pub fn new(carburetor_message_sender: Sender<LinkageToCarburetor>) -> Self {
        Self {
            gamepad_manager: GamepadManager::new(),
            carburetor_message_sender,
        }
    }
}
