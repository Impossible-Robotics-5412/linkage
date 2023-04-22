//! The prelude file provides a simple way to import often used code into your program.
//!
//! # Examples
//! ```
//! use linkage_rs::prelude::*;
//!
//! Robot::new()
//!     .run();
//! ```
pub use crate::gamepads::AssociatedGamepad;
pub use crate::robot::Robot;
pub use crate::state::RobotStateHandle;
pub use crate::subsystem::Subsystem;
