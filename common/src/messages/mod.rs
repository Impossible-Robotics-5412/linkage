mod error;

use error::MessageError;

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

#[derive(Debug, Clone, Copy)]
pub enum BackendToRuntimeMessage {
    Enable,
    Disable,
}

impl Message for BackendToRuntimeMessage {
    fn to_bytes(&self) -> Bytes {
        Bytes::from(*self)
    }
}

impl TryFrom<Bytes> for BackendToRuntimeMessage {
    type Error = MessageError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        match value {
            ENABLE_RUNTIME => Ok(Self::Enable),
            DISABLE_RUNTIME => Ok(Self::Disable),
            bytes => Err(MessageError::UnknownMessage(bytes)),
        }
    }
}

impl From<BackendToRuntimeMessage> for Bytes {
    fn from(value: BackendToRuntimeMessage) -> Self {
        match value {
            BackendToRuntimeMessage::Enable => ENABLE_RUNTIME,
            BackendToRuntimeMessage::Disable => DISABLE_RUNTIME,
        }
    }
}

// Runtime ------> Backend

#[derive(Debug, Clone, Copy)]
pub enum RuntimeToBackendMessage {
    Enabled,
    Disabled,
}

impl Message for RuntimeToBackendMessage {
    fn to_bytes(&self) -> Bytes {
        Bytes::from(*self)
    }
}

impl TryFrom<Bytes> for RuntimeToBackendMessage {
    type Error = MessageError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        match value {
            RUNTIME_IS_ENABLED => Ok(Self::Enabled),
            RUNTIME_IS_DISABLED => Ok(Self::Disabled),
            bytes => Err(MessageError::UnknownMessage(bytes)),
        }
    }
}

impl From<RuntimeToBackendMessage> for Bytes {
    fn from(value: RuntimeToBackendMessage) -> Self {
        match value {
            RuntimeToBackendMessage::Enabled => RUNTIME_IS_ENABLED,
            RuntimeToBackendMessage::Disabled => RUNTIME_IS_DISABLED,
        }
    }
}

// Frontend ------> Backend

#[derive(Debug, Clone, Copy)]
pub enum FrontendToBackendMessage {
    Enable,
    Disable,
}

impl Message for FrontendToBackendMessage {
    fn to_bytes(&self) -> Bytes {
        Bytes::from(*self)
    }
}

impl TryFrom<Bytes> for FrontendToBackendMessage {
    type Error = MessageError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        match value {
            ENABLE_BACKEND => Ok(Self::Enable),
            DISABLE_BACKEND => Ok(Self::Disable),
            bytes => Err(MessageError::UnknownMessage(bytes)),
        }
    }
}

impl From<FrontendToBackendMessage> for Bytes {
    fn from(value: FrontendToBackendMessage) -> Self {
        match value {
            FrontendToBackendMessage::Enable => ENABLE_BACKEND,
            FrontendToBackendMessage::Disable => DISABLE_BACKEND,
        }
    }
}

// Backend ------> Frontend

#[derive(Debug, Clone, Copy)]
pub enum BackendToFrontendMessage {
    Enabled,
    Disabled,
}

impl Message for BackendToFrontendMessage {
    fn to_bytes(&self) -> Bytes {
        Bytes::from(*self)
    }
}

impl TryFrom<Bytes> for BackendToFrontendMessage {
    type Error = MessageError;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        match value {
            BACKEND_IS_ENABLED => Ok(Self::Enabled),
            BACKEND_IS_DISABLED => Ok(Self::Disabled),
            bytes => Err(MessageError::UnknownMessage(bytes)),
        }
    }
}

impl From<BackendToFrontendMessage> for Bytes {
    fn from(value: BackendToFrontendMessage) -> Self {
        match value {
            BackendToFrontendMessage::Enabled => BACKEND_IS_ENABLED,
            BackendToFrontendMessage::Disabled => BACKEND_IS_DISABLED,
        }
    }
}
