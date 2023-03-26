mod error;

use error::MessageError;
use message_derive::Message;

/// An 8-byte array that serves as the common message sent between linkage programs.
pub type Bytes = [u8; 8];

// Cockpit-Frontend > Cockpit-Backend
/// Tells backend that its services should be enabled.
const ENABLE_BACKEND: Bytes = [0x00, 0, 0, 0, 0, 0, 0, 0];
/// Tells backend that its services should be disabled.
const DISABLE_BACKEND: Bytes = [0x01, 0, 0, 0, 0, 0, 0, 0];

// Cockpit-Frontend < Cockpit-Backend
/// Tells frontend that backend's services have been enabled.
const BACKEND_IS_ENABLED: Bytes = [0x08, 0, 0, 0, 0, 0, 0, 0];
/// Tells frontend that backend's services have been disabled.
const BACKEND_IS_DISABLED: Bytes = [0x09, 0, 0, 0, 0, 0, 0, 0];

// Cockpit-Backend > Runtime
/// Tells runtime that it should be enabled.
const ENABLE_RUNTIME: Bytes = [0x10, 0, 0, 0, 0, 0, 0, 0];
/// Tells runtime that it should be disabled.
const DISABLE_RUNTIME: Bytes = [0x11, 0, 0, 0, 0, 0, 0, 0];

// Cockpit-Backend < Runtime
/// Tells backend that runtime has been enabled.
const RUNTIME_IS_ENABLED: Bytes = [0x18, 0, 0, 0, 0, 0, 0, 0];
/// Tells backend that runtime has been disabled.
const RUNTIME_IS_DISABLED: Bytes = [0x19, 0, 0, 0, 0, 0, 0, 0];

// TODO: For future implementation of the messages between carburetor and linkage :)
// // Linkage-Lib > Carburetor
// // const MOTOR_INSTRUCTION = [0x40, data...] ;
// const QUERY_BATTERY: Bytes = [0x80, 0, 0, 0, 0, 0, 0, 0];
// const QUERY_CPU: Bytes = [0x81, 0, 0, 0, 0, 0, 0, 0];
// const QUERY_MEMORY: Bytes = [0x82, 0, 0, 0, 0, 0, 0, 0];
//
// // Linkage-Lib < Carburetor
// const BATTERY: Bytes = [0x90, 0, 0, 0, 0, 0, 0, 0];
// const CPU: Bytes = [0x91, 0, 0, 0, 0, 0, 0, 0];
// const MEMORY: Bytes = [0x92, 0, 0, 0, 0, 0, 0, 0];

/// The main Message trait, describing conversion from self to [`Bytes`].
pub trait Message: TryFrom<Bytes> + Into<Bytes> {
    fn to_bytes(&self) -> Bytes;
}

// Backend ------> Runtime
#[derive(Debug, Clone, Copy, Message)]
pub enum BackendToRuntimeMessage {
    #[message(ENABLE_RUNTIME)]
    Enable,
    #[message(DISABLE_RUNTIME)]
    Disable,
}

// Runtime ------> Backend
#[derive(Debug, Clone, Copy, Message)]
pub enum RuntimeToBackendMessage {
    #[message(RUNTIME_IS_ENABLED)]
    Enabled,
    #[message(RUNTIME_IS_DISABLED)]
    Disabled,
}

// Frontend ------> Backend
#[derive(Debug, Clone, Copy, Message)]
pub enum FrontendToBackendMessage {
    #[message(ENABLE_BACKEND)]
    Enable,
    #[message(DISABLE_BACKEND)]
    Disable,
}

// Backend ------> Frontend
#[derive(Debug, Clone, Copy, Message)]
pub enum BackendToFrontendMessage {
    #[message(BACKEND_IS_ENABLED)]
    Enabled,
    #[message(BACKEND_IS_DISABLED)]
    Disabled,
}

// Backend ------> Linkage Lib
#[derive(Debug, Clone, Copy)]
pub enum BackendToLinkage {
    GamepadInputEvent {
        gamepad_id: u8,
        event_type: u8,
        control: u8,
        value: u8,
    },
}

impl Message for BackendToLinkage {
    fn to_bytes(&self) -> Bytes {
        Bytes::from(*self)
    }
}

impl TryFrom<Bytes> for BackendToLinkage {
    type Error = MessageError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        match value {
            [0x20, 0, 0, 0, gamepad_id, event_type, control, value] => {
                Ok(Self::GamepadInputEvent {
                    gamepad_id,
                    event_type,
                    control,
                    value,
                })
            }
            bytes => Err(MessageError::UnknownMessage(bytes)),
        }
    }
}

impl From<BackendToLinkage> for Bytes {
    fn from(value: BackendToLinkage) -> Self {
        #[allow(unused_parens)]
        match value {
            BackendToLinkage::GamepadInputEvent {
                gamepad_id,
                event_type,
                control,
                value,
            } => [0x20, 0, 0, 0, gamepad_id, event_type, control, value],
        }
    }
}
