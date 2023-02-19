use std::time::Duration;

use crate::{PULSE_DELTA_US, PULSE_NEUTRAL_US};

/// A message type that has the following format:
///
///  - 0: u8 describing the instruction.
///  - 1: u8 describing the channel.
///  - 2..=3: empty (do not use, could be used for future additions).
///  - 4..=7: when when sending a float, these four bytes together form a big-endian f32.
pub(crate) type MessageBytes = [u8; 8];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum Direction {
    Forward,
    Neutral,
    Backward,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::Forward => "forward",
            Direction::Neutral => "neutral",
            Direction::Backward => "backward",
        };

        write!(f, "{s}")
    }
}

/// Contains a value between -1.0 and 1.0 representing a speed.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct Speed(f32);

impl Speed {
    /// Creates a new [`Speed`].
    ///
    /// `value` must be within -1.0..=1.0.
    ///  - -1.0 represents full speed reverse.
    ///  - 1.0 represents full speed forwards.
    ///  - 0.0 represents neutral speed, or stand-still.
    ///
    /// Returns [`None`] if the provided value is not within -1.0..=1.0.
    pub(crate) fn new(value: f32) -> Option<Self> {
        if (-1.0..=1.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Creates a new [`Speed`] with zero speed.
    pub(crate) fn neutral() -> Self {
        Self::new(0.0).unwrap()
    }

    /// Creates a new [`Speed`] with full forward force.
    pub(crate) fn forward() -> Self {
        Self::new(1.0).unwrap()
    }

    /// Creates a new [`Speed`] with full reverse force.
    pub(crate) fn backward() -> Self {
        Self::new(-1.0).unwrap()
    }

    /// Returns the duration of this [`Speed`].
    pub(crate) fn duration(&self) -> Duration {
        let micros = PULSE_NEUTRAL_US as i32 + (PULSE_DELTA_US as f32 * self.0) as i32;
        Duration::from_micros(micros as u64)
    }

    /// Returns the direction of this [`Speed`].
    ///
    /// # Panics
    ///
    /// Panics if the speed is invalid, that is, not within -1.0..=1.0.
    pub(crate) fn direction(&self) -> Direction {
        match self.0 {
            s if s < 0.0 => Direction::Backward,
            s if s > 0.0 => Direction::Forward,
            s if s == 0.0 => Direction::Neutral,
            s => panic!("invalid speed '{s}': not in range -1.0..=1.0"),
        }
    }
}

impl std::fmt::Display for Speed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) enum Instruction {
    Motor(MotorInstruction),
    Query, // This is a test query bound to 69
    Battery,
    Memory,
    Cpu,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MotorInstruction {
    channel: usize,
    speed: Speed,
}

impl MotorInstruction {
    /// Creates a new [`MotorInstruction`].
    ///
    /// This function returns [`None`] if the value of `speed` is not within -1.0..=1.0.
    ///
    /// This can happen when `speed` is not within that range, or when it is not a valid
    /// floating point number.
    pub(crate) fn new(channel: usize, speed: f32) -> Option<Self> {
        Some(Self {
            channel,
            speed: Speed::new(speed)?,
        })
    }

    /// Returns the channel of this [`MotorInstruction`].
    pub(crate) fn channel(&self) -> usize {
        self.channel
    }

    /// Returns the speed of this [`MotorInstruction`].
    pub(crate) fn speed(&self) -> Speed {
        self.speed
    }
}

/// Decodes the buffer of type and format [`MessageBytes`].
///
/// Returns [`None`] if the decoded speed is not valid. The speed is invalid when it is not within
/// -1.0..=1.0.
pub(crate) fn decode(buf: MessageBytes) -> Option<Instruction> {
    let instruction_type = buf[0];
    match instruction_type {
        0 => {
            let channel = buf[1] as usize;
            let speed_bytes: [u8; 4] = [buf[4], buf[5], buf[6], buf[7]];
            let speed = f32::from_be_bytes(speed_bytes);
            // We round the speed to prevent -0.00000000012098421 from not registering as neutral.
            let speed = (speed * 10_000.0).round() / 10_000.0;
            Some(Instruction::Motor(MotorInstruction::new(channel, speed)?))
        }
        100 => Some(Instruction::Battery),
        101 => Some(Instruction::Memory),
        102 => Some(Instruction::Cpu),
        69 => Some(Instruction::Query),
        // Not yet implemented.
        _ => None,
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_decode() {
//         assert_eq!(decode([0, 0, 0, 0, 0]), Instruction::new(0, -1.0));
//         assert_eq!(
//             decode([[100, 0, 0, 0], (0.5_f32).to_be_bytes()].concat()),
//             Instruction::new(100, 0.0)
//         );
//         assert_eq!(decode([200, 0, 0, 0, u8::MAX]), Instruction::new(200, 1.0));
//     }
// }
