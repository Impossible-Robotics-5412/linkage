use super::gamepad::{EventType, Gamepad, GamepadData, GamepadId};
use messaging::CockpitToLinkage;

/// Represents a gamepad index.
#[repr(usize)]
pub enum AssociatedGamepad {
    Primary = 0,
    Secondary = 1,
}

/// Manages gamepad devices, providing methods to access and handle gamepad input events.
#[derive(Debug)]
pub struct GamepadManager {
    gamepads: Vec<Option<GamepadData>>,
}

impl GamepadManager {
    /// Returns a gamepad device associated with the specified [`AssociatedGamepad`], if present.
    ///
    /// # Parameters
    /// - `associated_gamepad`: The [`AssociatedGamepad`] to get the gamepad device for.
    ///
    /// # Returns
    /// An `Option<G>` containing the gamepad device of type `G` if it exists, or `None` otherwise.
    pub fn get<G: Gamepad>(&self, associated_gamepad: AssociatedGamepad) -> Option<G> {
        if let Some(Some(gamepad)) = self.gamepads.get(associated_gamepad as usize) {
            return Some(G::new(gamepad.to_owned()));
        }
        None
    }

    pub(crate) fn new() -> Self {
        Self {
            gamepads: Vec::new(),
        }
    }

    pub(crate) fn handle_cockpit_message(&mut self, message: CockpitToLinkage) {
        match message {
            CockpitToLinkage::GamepadInputEvent {
                gamepad_id,
                event_type,
                control,
                value,
            } => {
                let mut insert_if_not_found = || -> bool {
                    if self.index_from_id(gamepad_id).is_none() {
                        let mut gamepad = GamepadData::new(gamepad_id);
                        gamepad
                            .handle_cockpit_message(event_type, control, value)
                            .unwrap();
                        self.insert_gamepad(gamepad);
                        return true;
                    }
                    false
                };

                match EventType::try_from(event_type) {
                    Ok(event_type) => match event_type {
                        EventType::ButtonChanged | EventType::AxisChanged => {
                            if insert_if_not_found() {
                                return;
                            }

                            if let Some(Some(gamepad)) =
                                self.gamepads.iter_mut().find(|g| match g {
                                    Some(g) => g.gamepad_id() == gamepad_id,
                                    _ => false,
                                })
                            {
                                gamepad
                                    .handle_cockpit_message(event_type as u8, control, value)
                                    .unwrap()
                            }
                        }
                        EventType::Connected => {
                            insert_if_not_found();
                        }
                        EventType::Disconnected => {
                            if let Some(index) = self.index_from_id(gamepad_id) {
                                self.set_item(index, None);
                            }
                        }
                    },
                    Err(_) => log::error!("Invalid event_type: {event_type}"),
                }
            }
        }
    }

    fn set_item(&mut self, index: usize, item: Option<GamepadData>) {
        _ = std::mem::replace(&mut self.gamepads[index], item);
    }

    fn insert_gamepad(&mut self, gamepad: GamepadData) {
        if let Some(index) = self.gamepads.iter().position(|o| o.is_none()) {
            self.set_item(index, Some(gamepad));
        } else {
            self.gamepads.push(Some(gamepad));
        }
    }

    fn index_from_id(&self, id: GamepadId) -> Option<usize> {
        self.gamepads.iter().position(|g| match g {
            Some(g) => g.gamepad_id() == id,
            _ => false,
        })
    }
}
