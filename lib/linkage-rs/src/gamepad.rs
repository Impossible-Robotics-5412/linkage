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
    gamepad_id: GamepadId,
    buttons: HashMap<u8, u8>,
    axis: HashMap<u8, u8>,
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

pub struct PsController {
    gamepad_data: GamepadData,
}

impl PsController {
    pub fn new(gamepad_data: GamepadData) -> Self {
        Self { gamepad_data }
    }

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

    pub fn triangle(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::North as u8)
    }
    pub fn square(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::West as u8)
    }
    pub fn cross(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::South as u8)
    }
    pub fn circle(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::East as u8)
    }
    pub fn dpad_up(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::DpadUp as u8)
    }
    pub fn dpad_down(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::DpadDown as u8)
    }
    pub fn dpad_left(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::DpadLeft as u8)
    }
    pub fn dpad_right(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::DpadRight as u8)
    }
    pub fn left_bumper(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::LeftTrigger as u8)
    }
    pub fn right_bumper(&self) -> bool {
        self.control_button_value(
            &self.gamepad_data.buttons,
            ButtonControl::RightTrigger as u8,
        )
    }
    pub fn left_trigger(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.buttons,
            ButtonControl::LeftTrigger2 as u8,
            (0f32, 1f32),
        )
    }
    pub fn right_trigger(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.buttons,
            ButtonControl::RightTrigger2 as u8,
            (0f32, 1f32),
        )
    }
    pub fn left_joystick_x(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.axis,
            AxisControl::LeftStickX as u8,
            (-1f32, 1f32),
        )
    }
    pub fn left_joystick_y(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.axis,
            AxisControl::LeftStickY as u8,
            (-1f32, 1f32),
        )
    }
    pub fn left_joystick_button(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::LeftThumb as u8)
    }
    pub fn right_joystick_x(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.axis,
            AxisControl::RightStickX as u8,
            (-1f32, 1f32),
        )
    }
    pub fn right_joystick_y(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.axis,
            AxisControl::RightStickY as u8,
            (-1f32, 1f32),
        )
    }
    pub fn right_joystick_button(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::RightThumb as u8)
    }
    pub fn share(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::Select as u8)
    }
    pub fn options(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::Start as u8)
    }
    pub fn home(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::Mode as u8)
    }
}
