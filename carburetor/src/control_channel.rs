#[cfg(target_arch = "armv7")]
use crate::{PERIOD_MS, PULSE_NEUTRAL_US};
#[cfg(target_arch = "armv7")]
use rppal::{
    pwm::{Channel, Polarity, Pwm},
    system::DeviceInfo,
};
#[cfg(target_arch = "armv7")]
use std::time::Duration;

use std::sync::mpsc::Receiver;
use std::{error::Error, fmt::Display};

use crate::Speed;

#[cfg(not(target_arch = "armv7"))]
pub(crate) enum Channel {
    Pwm0,
    Pwm1,
}
#[cfg(not(target_arch = "armv7"))]
impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pwm0 => write!(f, "Pwm0"),
            Self::Pwm1 => write!(f, "Pwm1"),
        }
    }
}

pub(crate) fn control_channel(
    pwm_channel: Channel,
    receiver: Receiver<Speed>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(target_arch = "armv7")]
    {
        eprintln!(
            "Pulse-width modulation on channel {pwm_channel} on a {}.",
            DeviceInfo::new()?.model()
        );

        let pwm = Pwm::with_period(
            pwm_channel,
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(PULSE_NEUTRAL_US),
            Polarity::Normal,
            true,
        )?;
    }

    while let Ok(speed) = receiver.recv() {
        let before = std::time::Instant::now();
        #[cfg(target_arch = "armv7")]
        pwm.set_pulse_width(speed.duration()).unwrap();
        let after = std::time::Instant::now();
        let delta = after - before;
        let direction = speed.direction();
        eprintln!("{pwm_channel}: received {speed} ({direction}). Executed in {delta:?}.");
    }

    Ok(())
}
