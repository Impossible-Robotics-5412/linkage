mod error;

use error::MessageError;

/// An 8-byte array that serves as the common message sent between linkage programs.
pub type Bytes = [u8; 8];

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

// Backend ------> Linkage Lib
#[derive(Debug, Clone, Copy)]
pub enum CockpitToLinkage {
    GamepadInputEvent {
        gamepad_id: u8,
        event_type: u8,
        control: u8,
        value: u8,
    },
}

impl Message for CockpitToLinkage {
    fn to_bytes(&self) -> Bytes {
        Bytes::from(*self)
    }
}

impl TryFrom<Bytes> for CockpitToLinkage {
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

impl From<CockpitToLinkage> for Bytes {
    fn from(value: CockpitToLinkage) -> Self {
        #[allow(unused_parens)]
        match value {
            CockpitToLinkage::GamepadInputEvent {
                gamepad_id,
                event_type,
                control,
                value,
            } => [0x20, 0, 0, 0, gamepad_id, event_type, control, value],
        }
    }
}
