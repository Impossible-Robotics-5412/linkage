mod error;

use error::MessageError;
use serde::{Deserialize, Serialize};

/// An 8-byte array that serves as the common message sent between linkage programs.
pub type Bytes = [u8; 8];

/// The main Message trait, describing conversion from self to [`Bytes`].
pub trait Message: TryFrom<Bytes> + Into<Bytes> {
    fn to_bytes(&self) -> Bytes;
}

// Cockpit ------> Linkage Lib
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
        match value {
            // TODO: We could use a f32 byte representation for the value
            //       just like we do with LinkageToCarburetor::MotorInstruction
            CockpitToLinkage::GamepadInputEvent {
                gamepad_id,
                event_type,
                control,
                value,
            } => [0x20, 0, 0, 0, gamepad_id, event_type, control, value],
        }
    }
}

// Linkage ------> Carburetor
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LinkageToCarburetor {
    MotorInstruction { channel: u8, speed: f32 },
}

impl TryFrom<Bytes> for LinkageToCarburetor {
    type Error = MessageError;

    /// Decodes the buffer of type and format [`MessageBytes`].
    ///
    /// Returns [`None`] if the decoded speed is not valid. The speed is invalid when it is not within
    /// -1.0..=1.0.
    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        match value {
            [0x40, channel, 0, 0, speed_byte_1, speed_byte_2, speed_byte_3, speed_byte_4] => {
                let speed_bytes: [u8; 4] = [speed_byte_1, speed_byte_2, speed_byte_3, speed_byte_4];
                let speed = f32::from_be_bytes(speed_bytes);
                // We round the speed to prevent -0.00000000012098421 from not registering as neutral.
                let speed = (speed * 10_000.0).round() / 10_000.0;
                Ok(LinkageToCarburetor::MotorInstruction { channel, speed })
            }
            bytes => Err(MessageError::UnknownMessage(bytes)),
        }
    }
}

impl From<LinkageToCarburetor> for Bytes {
    fn from(value: LinkageToCarburetor) -> Self {
        match value {
            LinkageToCarburetor::MotorInstruction { channel, speed } => {
                let speed = speed.to_be_bytes();
                [0x40, channel, 0, 0, speed[0], speed[1], speed[2], speed[3]]
            }
        }
    }
}

impl Message for LinkageToCarburetor {
    fn to_bytes(&self) -> Bytes {
        Bytes::from(*self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CockpitToLinkage, LinkageToCarburetor, Message};

    #[test]
    fn cockpit_to_linkage_try_from_bytes() {
        let message = CockpitToLinkage::try_from([0x20, 0, 0, 0, 42, 43, 44, 45]).unwrap();

        assert_eq!(
            message,
            CockpitToLinkage::GamepadInputEvent {
                gamepad_id: 42,
                event_type: 43,
                control: 44,
                value: 45,
            }
        )
    }

    #[test]
    fn cockpit_to_linkage_try_from_bytes_wrong_unused_bytes() {
        let result = CockpitToLinkage::try_from([0x20, 69, 69, 69, 42, 43, 44, 45]);
        assert!(result.is_err());
    }

    #[test]
    fn cockpit_to_linkage_try_from_bytes_wrong_instruction() {
        let result = CockpitToLinkage::try_from([0x10, 0, 0, 0, 42, 43, 44, 45]);
        assert!(result.is_err());
    }

    #[test]
    fn bytes_from_cockpit_to_linkage() {
        let bytes = CockpitToLinkage::GamepadInputEvent {
            gamepad_id: 42,
            event_type: 43,
            control: 44,
            value: 45,
        }
        .to_bytes();

        assert_eq!(bytes, [0x20, 0, 0, 0, 42, 43, 44, 45])
    }

    #[test]
    fn linkage_to_carburetor_from_bytes() {
        let message = LinkageToCarburetor::try_from([0x40, 1, 0, 0, 63, 49, 183, 23]).unwrap();

        assert_eq!(
            message,
            LinkageToCarburetor::MotorInstruction {
                channel: 1,
                speed: 0.69420f32
            }
        )
    }

    #[test]
    fn linkage_to_carburetor_from_bytes_wrong_instruction() {
        let result = LinkageToCarburetor::try_from([0x10, 1, 0, 0, 63, 49, 183, 23]);
        assert!(result.is_err());
    }

    #[test]
    fn linkage_to_carburetor_from_bytes_wrong_unused_bytes() {
        let result = LinkageToCarburetor::try_from([0x40, 1, 69, 69, 63, 49, 183, 23]);
        assert!(result.is_err());
    }

    #[test]
    fn bytes_from_linkage_to_carburetor() {
        let bytes = LinkageToCarburetor::MotorInstruction {
            channel: 1,
            speed: 0.69420f32,
        }
        .to_bytes();

        assert_eq!(bytes, [0x40, 1, 0, 0, 63, 49, 183, 23])
    }
}
