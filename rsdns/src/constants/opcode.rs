use crate::{Error, ProtocolError};
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

/// Query opcode.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, EnumIter, EnumString, IntoStaticStr, Hash,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum OpCode {
    /// a standard query
    QUERY = 0,
    /// an inverse query
    IQUERY = 1,
    /// a server status request
    STATUS = 2,
}

impl OpCode {
    /// Converts `OpCode` to a static string.
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}

impl TryFrom<u8> for OpCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let me = match value {
            0 => OpCode::QUERY,
            1 => OpCode::IQUERY,
            2 => OpCode::STATUS,
            _ => return Err(Error::from(ProtocolError::ReservedOpCode(value))),
        };

        Ok(me)
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_try_from() {
        for opcode in OpCode::iter() {
            assert_eq!(opcode, OpCode::try_from(opcode as u8).unwrap());
        }

        assert!(matches!(
            OpCode::try_from(128),
            Err(Error::ProtocolError(ProtocolError::ReservedOpCode(128)))
        ));
    }
}
