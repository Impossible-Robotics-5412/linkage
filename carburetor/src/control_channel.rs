use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::system::DeviceInfo;

use std::error::Error;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crate::{Speed, PERIOD_MS, PULSE_NEUTRAL_US};

pub(crate) fn control_channel(
    pwm_channel: Channel,
    receiver: Receiver<Speed>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
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

    while let Ok(speed) = receiver.recv() {
        let before = std::time::Instant::now();
        pwm.set_pulse_width(speed.duration()).unwrap();
        let after = std::time::Instant::now();
        let delta = after - before;
        let direction = speed.direction();
        eprintln!("{pwm_channel}: received {speed} ({direction}). Executed in {delta:?}.");
    }

    Ok(())
}
