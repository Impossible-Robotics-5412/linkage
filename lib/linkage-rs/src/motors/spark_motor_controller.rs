use crate::state::RobotStateHandle;
pub struct SparkMotorController {
    robot_state: RobotStateHandle,
    channel: u8,
}

impl SparkMotorController {
    pub fn new(robot_state: RobotStateHandle, channel: u8) -> Self {
        Self {
            robot_state,
            channel,
        }
    }

    pub fn set_speed_percentage(&self, speed: f32) {
        let sender = self
            .robot_state
            .lock()
            .unwrap()
            .carburetor_message_sender
            .clone();

        sender
            .send(common::messages::LinkageToCarburetor::MotorInstruction {
                channel: self.channel,
                speed,
            })
            .unwrap();
    }
}
