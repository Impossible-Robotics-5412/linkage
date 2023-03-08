use super::Bytes;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeMessage(Bytes);

impl From<RuntimeMessage> for Bytes {
    fn from(value: RuntimeMessage) -> Self {
        value.0
    }
}

// impl Message for RuntimeMessage {
//     fn to_bytes(&self) -> Bytes {
//         self.0
//     }
// }
