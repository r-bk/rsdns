use crate::Error;
use std::convert::TryFrom;
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

/// DNS query OPCODE.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr, Hash)]
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
    pub fn as_str(self) -> &'static str {
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
            _ => return Err(Error::UnknownOpCode(value)),
        };

        Ok(me)
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
            Err(Error::UnknownOpCode(128))
        ));
    }
}
