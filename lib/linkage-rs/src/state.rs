//! Shared data used to access data like gamepad input within for example subsystems.

use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use messaging::LinkageToCarburetor;

use crate::gamepads::GamepadManager;

/// A thread-safe handle to a [`RobotState`].
pub type RobotStateHandle = Arc<Mutex<RobotState>>;

/// Used to access data like gamepad input.
#[derive(Debug)]
pub struct RobotState {
    /// Responsible for managing the connected [Gamepad][`crate::gamepads::gamepad::Gamepad`]s.
    pub gamepad_manager: GamepadManager,
    pub(crate) carburetor_message_sender: Sender<LinkageToCarburetor>,
}

impl RobotState {
    pub(crate) fn new(carburetor_message_sender: Sender<LinkageToCarburetor>) -> Self {
        Self {
            gamepad_manager: GamepadManager::new(),
            carburetor_message_sender,
        }
    }
}
