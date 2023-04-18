use super::Bytes;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageError {
    UnknownMessage(Bytes),
}

impl std::error::Error for MessageError {}

impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}
