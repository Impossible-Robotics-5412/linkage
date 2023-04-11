use std::collections::HashMap;
use std::io;

pub type GamepadId = u8;

#[repr(u8)]
pub enum EventType {
    ButtonChanged = 0,
    AxisChanged = 1,
    Connected = 2,
    Disconnected = 3,
}

#[repr(u8)]
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

#[repr(u8)]
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

#[derive(Default, Debug, Clone)]
pub struct GamepadData {
    pub(crate) gamepad_id: GamepadId,
    pub(crate) buttons: HashMap<u8, u8>,
    pub(crate) axis: HashMap<u8, u8>,
}

impl GamepadData {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn handle_cockpit_message(
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

    pub fn gamepad_id(&self) -> u8 {
        self.gamepad_id
    }
}

pub trait Gamepad {
    // FIXME: Should this be using a From trait?
    fn new(gamepad_data: GamepadData) -> Self;

    fn control_button_value(&self, map: &HashMap<u8, u8>, control: u8) -> bool {
        match map.get(&control) {
            Some(value) => value > &127,
            None => false,
        }
    }

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
