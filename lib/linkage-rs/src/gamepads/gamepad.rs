use std::collections::HashMap;
use std::io::{self, ErrorKind};

use enum_iterator::Sequence;

const AXIS_DEFAULT: u8 = 127;
const BUTTON_DEFAULT: u8 = 0;

pub type GamepadId = u8;

impl TryFrom<u8> for EventType {
    type Error = io::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == EventType::ButtonChanged as u8 {
            Ok(EventType::ButtonChanged)
        } else if value == EventType::AxisChanged as u8 {
            Ok(EventType::AxisChanged)
        } else if value == EventType::Connected as u8 {
            Ok(EventType::Connected)
        } else if value == EventType::Disconnected as u8 {
            Ok(EventType::Disconnected)
        } else {
            Err(io::Error::new(
                ErrorKind::InvalidInput,
                "Failed to convert u8 into EventType",
            ))
        }
    }
}

/// Represents different button controls on a gamepad.
#[repr(u8)]
#[allow(dead_code)]
#[derive(Sequence)]
pub enum ButtonControl {
    // Action Pad
    South = 1,
    East = 2,
    North = 4,
    West = 5,
    C = 3,
    Z = 6,
    // Triggers
    LeftTrigger = 7,
    LeftTrigger2 = 9,
    RightTrigger = 8,
    RightTrigger2 = 10,
    // Menu Pad
    Select = 11,
    Start = 12,
    Mode = 13,
    // Sticks
    LeftThumb = 14,
    RightThumb = 15,
    // D-Pad
    DpadUp = 16,
    DpadDown = 17,
    DpadLeft = 18,
    DpadRight = 19,

    Unknown = 0,
}

/// Represents different axis controls on a gamepad.
#[repr(u8)]
#[allow(dead_code)]
#[derive(Sequence)]
pub enum AxisControl {
    LeftStickX = 1,
    LeftStickY = 2,
    LeftZ = 3,
    RightStickX = 4,
    RightStickY = 5,
    RightZ = 6,
    DpadX = 7,
    DpadY = 8,
    Unknown = 0,
}

/// Represents the data for a specific gamepad.
#[derive(Default, Debug, Clone)]
pub struct GamepadData {
    gamepad_id: GamepadId,
    pub buttons: HashMap<u8, u8>,
    pub axis: HashMap<u8, u8>,
}

impl GamepadData {
    pub(crate) fn new(gamepad_id: GamepadId) -> Self {
        let mut axis = HashMap::<u8, u8>::new();
        let mut buttons = HashMap::<u8, u8>::new();

        for control in enum_iterator::all::<AxisControl>() {
            axis.insert(control as u8, AXIS_DEFAULT);
        }

        for control in enum_iterator::all::<ButtonControl>() {
            buttons.insert(control as u8, BUTTON_DEFAULT);
        }

        Self {
            gamepad_id,
            buttons,
            axis,
        }
    }

    pub(crate) fn handle_cockpit_message(
        &mut self,
        event_type: u8,
        control: u8,
        value: u8,
    ) -> io::Result<()> {
        if event_type == EventType::ButtonChanged as u8 {
            self.buttons.insert(control, value);
        } else if event_type == EventType::AxisChanged as u8 {
            self.axis.insert(control, value);
        }

        Ok(())
    }

    /// Returns the gamepad ID.
    ///
    /// # Returns
    /// The [`GamepadId`] associated with this [`GamepadData`] instance.
    pub fn gamepad_id(&self) -> u8 {
        self.gamepad_id
    }
}

/// Represents a specific Gamepad device. This makes sure you can call functions
/// With the correct mapping. For examples, The cardinal buttons on a X-Box controller are different
/// from a PS controller.
pub trait Gamepad {
    /// Creates a new instance of a gamepad device.
    ///
    /// # Parameters
    /// - `gamepad_data`: The [`GamepadData`] to be associated with the gamepad device.
    ///
    /// # Returns
    /// A new instance of the implemented gamepad device.
    fn new(gamepad_data: GamepadData) -> Self;

    /// Returns the boolean state of a button control.
    ///
    /// # Parameters
    /// - `map`: A reference to the map with the control you want to check.
    /// - `control`: The control ID for the button to be checked.
    ///
    /// # Returns
    /// A boolean value indicating whether the specified button is pressed (true) or not (false).
    fn control_button_value(&self, map: &HashMap<u8, u8>, control: u8) -> bool {
        match map.get(&control) {
            Some(value) => value > &127,
            None => false,
        }
    }

    /// Returns the float value of an axis control, mapped to the specified range.
    ///
    /// # Parameters
    /// - `map`: A reference to the map with the control you want to check.
    /// - `control`: The control ID for the axis to be checked.
    /// - `axis_range`: A tuple representing the range `(min, max)` to map the axis value to.
    ///
    /// # Returns
    /// A float value representing the axis value mapped to the specified range.
    fn control_axis_value(
        &self,
        map: &HashMap<u8, u8>,
        control: u8,
        axis_range: (f32, f32),
    ) -> f32 {
        match map.get(&control) {
            Some(value) => {
                fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
                    to_range.0
                        + (s - from_range.0) * (to_range.1 - to_range.0)
                            / (from_range.1 - from_range.0)
                }

                let clamped = f32::clamp(value.to_owned() as f32, 0f32, 255f32);
                map_range((0f32, 255f32), axis_range, clamped)
            }
            None => 0f32,
        }
    }
}

/// Represents different types of gamepad events.
#[repr(u8)]
#[allow(dead_code)]
pub(crate) enum EventType {
    ButtonChanged = 0,
    AxisChanged = 1,
    Connected = 2,
    Disconnected = 3,
}
