mod error;

use error::MessageError;

/// An 8-byte array that serves as the common message sent between linkage programs.
pub type Bytes = [u8; 8];

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
        match value {
            // FIXME: We could use a f32 byte representation for the value
            //        just like we do with LinkageToCarburetor::MotorInstruction
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
#[derive(Debug, Clone, Copy)]
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
