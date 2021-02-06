use crate::RsDnsError;
use std::convert::TryFrom;
use strum_macros::EnumIter;

/// DNS query OPCODE.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum Opcode {
    /// a standard query
    QUERY = 0,
    /// an inverse query
    IQUERY = 1,
    /// a server status request
    STATUS = 2,
}

impl TryFrom<u8> for Opcode {
    type Error = RsDnsError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let me = match value {
            0 => Opcode::QUERY,
            1 => Opcode::IQUERY,
            2 => Opcode::STATUS,
            _ => return Err(RsDnsError::ProtocolUnknownOpcode(value)),
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
        for opcode in Opcode::iter() {
            assert_eq!(opcode, Opcode::try_from(opcode as u8).unwrap());
        }

        assert!(matches!(
            Opcode::try_from(128),
            Err(RsDnsError::ProtocolUnknownOpcode(128))
        ));
    }
}
