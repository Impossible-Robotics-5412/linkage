use std::error::Error;
use std::sync::mpsc::Receiver;

#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
use rppal::pwm::{Channel, Polarity, Pwm};

use crate::Speed;

#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
pub(crate) enum Channel {
    Pwm0,
    Pwm1,
}

#[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
impl std::fmt::Display for Channel {
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
    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    let pwm = Pwm::with_period(
        pwm_channel,
        std::time::Duration::from_millis(crate::PERIOD_MS),
        std::time::Duration::from_micros(crate::PULSE_NEUTRAL_US),
        Polarity::Normal,
        true,
    )?;

    while let Ok(speed) = receiver.recv() {
        let before = std::time::Instant::now();
        #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
        pwm.set_pulse_width(speed.duration()).unwrap();
        let after = std::time::Instant::now();
        let delta = after - before;
        let direction = speed.direction();
        log::trace!("{pwm_channel}: received {speed} ({direction}). Executed in {delta:?}.");
    }

    Ok(())
}
