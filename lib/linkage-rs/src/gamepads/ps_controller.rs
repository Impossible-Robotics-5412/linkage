use super::gamepad::{AxisControl, ButtonControl, Gamepad, GamepadData};
use crate::gamepads::gamepad::GamepadId;

/// Represents a PlayStation controller.
pub struct PsController {
    gamepad_data: GamepadData,
}

impl Gamepad for PsController {
    /// Creates a new [`PsController`] instance with the given [GamepadData][`crate::gamepads::gamepad::GamepadData`].
    ///
    /// # Parameters
    /// - `gamepad_data`: The [GamepadData][`crate::gamepads::gamepad::GamepadData`] to be associated with the PlayStation controller.
    ///
    /// # Returns
    /// A new [`PsController`] instance.
    fn new(gamepad_data: GamepadData) -> Self {
        Self { gamepad_data }
    }
}

impl PsController {
    /// Returns the [GamepadId][`crate::gamepads::gamepad::GamepadId`] of the PlayStation controller.
    ///
    /// # Returns
    /// The [GamepadId][`crate::gamepads::gamepad::GamepadId`] of the PlayStation controller.
    pub fn id(&self) -> GamepadId {
        self.gamepad_data.gamepad_id()
    }

    /// Returns the state of the triangle button.
    ///
    /// # Returns
    /// `true` if the Triangle button is pressed, `false` otherwise.
    pub fn triangle(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::North as u8)
    }

    /// Returns the state of the square button.
    ///
    /// # Returns
    /// `true` if the square button is pressed, `false` otherwise.
    pub fn square(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::West as u8)
    }

    /// Returns the state of the cross button.
    ///
    /// # Returns
    /// `true` if the cross button is pressed, `false` otherwise.
    pub fn cross(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::South as u8)
    }

    /// Returns the state of the circle button.
    ///
    /// # Returns
    /// `true` if the circle button is pressed, `false` otherwise.
    pub fn circle(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::East as u8)
    }

    /// Returns the state of the dpad's up button.
    ///
    /// # Returns
    /// `true` if the dpad's up button is pressed, `false` otherwise.
    pub fn dpad_up(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::DpadUp as u8)
    }

    /// Returns the state of the dpad's down button.
    ///
    /// # Returns
    /// `true` if the dpad's down button is pressed, `false` otherwise.
    pub fn dpad_down(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::DpadDown as u8)
    }

    /// Returns the state of the dpad's left button.
    ///
    /// # Returns
    /// `true` if the dpad's left button is pressed, `false` otherwise.
    pub fn dpad_left(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::DpadLeft as u8)
    }

    /// Returns the state of the dpad's right button.
    ///
    /// # Returns
    /// `true` if the dpad's right button is pressed, `false` otherwise.
    pub fn dpad_right(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::DpadRight as u8)
    }

    /// Returns the state of the left bumper button.
    ///
    /// # Returns
    /// `true` if the left bumper button is pressed, `false` otherwise.
    pub fn left_bumper(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::LeftTrigger as u8)
    }

    /// Returns the state of the right bumper button.
    ///
    /// # Returns
    /// `true` if the right bumper button is pressed, `false` otherwise.
    pub fn right_bumper(&self) -> bool {
        self.control_button_value(
            &self.gamepad_data.buttons,
            ButtonControl::RightTrigger as u8,
        )
    }

    /// Returns the state of the left trigger.
    ///
    /// # Returns
    /// A value between 0.0 and 1.0 representing not pressed or fully pressed respectively.
    pub fn left_trigger(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.buttons,
            ButtonControl::LeftTrigger2 as u8,
            (0f32, 1f32),
        )
    }

    /// Returns the state of the right trigger.
    ///
    /// # Returns
    /// A value between 0.0 and 1.0 representing not pressed or fully pressed respectively.
    pub fn right_trigger(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.buttons,
            ButtonControl::RightTrigger2 as u8,
            (0f32, 1f32),
        )
    }

    /// Returns the x-axis of the left joystick
    ///
    /// # Returns
    /// A value between -1.0 and 1.0 representing completely left or completely right
    /// respectively with 0.0 being the center.
    pub fn left_joystick_x(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.axis,
            AxisControl::LeftStickX as u8,
            (-1f32, 1f32),
        )
    }

    /// Returns the y-axis of the left joystick
    ///
    /// # Returns
    /// A value between -1.0 and 1.0 representing completely up or completely down
    /// respectively with 0.0 being the center.
    pub fn left_joystick_y(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.axis,
            AxisControl::LeftStickY as u8,
            (-1f32, 1f32),
        )
    }

    /// Returns the state of the left joystick button.
    ///
    /// # Returns
    /// `true` if the left joystick button is pressed, `false` otherwise.
    pub fn left_joystick_button(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::LeftThumb as u8)
    }

    /// Returns the y-axis of the right joystick
    ///
    /// # Returns
    /// A value between -1.0 and 1.0 representing completely up or completely down
    /// respectively with 0.0 being the center.
    pub fn right_joystick_x(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.axis,
            AxisControl::RightStickX as u8,
            (-1f32, 1f32),
        )
    }

    /// Returns the y-axis of the right joystick
    ///
    /// # Returns
    /// A value between -1.0 and 1.0 representing completely up or completely down
    /// respectively with 0.0 being the center.
    pub fn right_joystick_y(&self) -> f32 {
        self.control_axis_value(
            &self.gamepad_data.axis,
            AxisControl::RightStickY as u8,
            (-1f32, 1f32),
        )
    }

    /// Returns the state of the right joystick button.
    ///
    /// # Returns
    /// `true` if the right joystick button is pressed, `false` otherwise.
    pub fn right_joystick_button(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::RightThumb as u8)
    }

    /// Returns the state of share button.
    ///
    /// # Returns
    /// `true` if the share button is pressed, `false` otherwise.
    pub fn share(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::Select as u8)
    }

    /// Returns the state of options button.
    ///
    /// # Returns
    /// `true` if the options button is pressed, `false` otherwise.
    pub fn options(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::Start as u8)
    }

    /// Returns the state of home button.
    ///
    /// # Returns
    /// `true` if the home button is pressed, `false` otherwise.
    pub fn home(&self) -> bool {
        self.control_button_value(&self.gamepad_data.buttons, ButtonControl::Mode as u8)
    }
}
