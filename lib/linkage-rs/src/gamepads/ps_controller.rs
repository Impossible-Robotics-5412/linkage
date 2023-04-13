use super::gamepad::{AxisControl, ButtonControl, Gamepad, GamepadData};

pub struct PsController {
    gamepad_data: GamepadData,
}

impl Gamepad for PsController {
    fn new(gamepad_data: GamepadData) -> Self {
        Self { gamepad_data }
    }
}

impl PsController {
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
