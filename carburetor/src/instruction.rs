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
    #[allow(dead_code)]
    pub(crate) fn forward() -> Self {
        Self::new(1.0).unwrap()
    }

    /// Creates a new [`Speed`] with full reverse force.
    #[allow(dead_code)]
    pub(crate) fn backward() -> Self {
        Self::new(-1.0).unwrap()
    }

    /// Returns the duration of this [`Speed`].
    #[allow(dead_code)]
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
