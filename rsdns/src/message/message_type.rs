/// Message type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum MessageType {
    /// A query message.
    Query,
    /// A response message.
    Response,
}

impl From<bool> for MessageType {
    fn from(value: bool) -> Self {
        if value {
            Self::Response
        } else {
            Self::Query
        }
    }
}

impl From<MessageType> for bool {
    fn from(mt: MessageType) -> Self {
        match mt {
            MessageType::Query => false,
            MessageType::Response => true,
        }
    }
}
