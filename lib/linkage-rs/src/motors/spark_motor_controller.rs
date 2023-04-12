use crate::state::RobotStateHandle;
pub struct SparkMotorController {
    state: RobotStateHandle,
    channel: u8,
}

impl SparkMotorController {
    pub fn new(state: RobotStateHandle, channel: u8) -> Self {
        Self { state, channel }
    }

    pub fn set_speed_percentage(&self, speed: f32) {
        let sender = self.state.lock().unwrap().carburetor_message_sender.clone();

        sender
            .send(common::messages::LinkageToCarburetor::MotorInstruction {
                channel: self.channel,
                speed,
            })
            .unwrap();
    }
}
