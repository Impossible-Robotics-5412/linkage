#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
use crate::{PERIOD_MS, PULSE_NEUTRAL_US};
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
use rppal::{
    pwm::{Channel, Polarity, Pwm},
    system::DeviceInfo,
};
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
use std::time::Duration;

use std::error::Error;
use std::sync::mpsc::Receiver;

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
        Duration::from_millis(PERIOD_MS),
        Duration::from_micros(PULSE_NEUTRAL_US),
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
        eprintln!("{pwm_channel}: received {speed} ({direction}). Executed in {delta:?}.");
    }

    Ok(())
}
