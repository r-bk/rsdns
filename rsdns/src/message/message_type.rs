use std::fmt::{self, Display, Formatter};
use strum_macros::IntoStaticStr;

/// Message type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, IntoStaticStr)]
pub enum MessageType {
    /// A query message.
    Query,
    /// A response message.
    Response,
}

impl MessageType {
    /// Converts MessageType to a static string.
    #[inline]
    pub fn as_str(self) -> &'static str {
        self.into()
    }
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

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
