//! Gamepads are input devices to control your robot.

pub mod gamepad;
mod gamepad_manager;
mod ps_controller;

pub use gamepad::Gamepad;
pub use gamepad_manager::{AssociatedGamepad, GamepadManager};
pub use ps_controller::PsController;
