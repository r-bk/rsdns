use crate::{Error, Result};
use std::fmt::{self, Display, Formatter};

/// Query opcodes.
///
/// [RFC 1035 section 4.1.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.1)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OpCode {
    /// a standard query
    Query = 0,
    /// an inverse query
    IQuery = 1,
    /// a server status request
    Status = 2,
}

impl OpCode {
    /// Array of all discriminants in this enum.
    #[cfg(test)]
    pub const VALUES: [OpCode; 3] = [OpCode::Query, OpCode::IQuery, OpCode::Status];

    /// Converts `OpCode` to a static string.
    pub fn to_str(self) -> &'static str {
        match self {
            OpCode::Query => "QUERY",
            OpCode::IQuery => "IQUERY",
            OpCode::Status => "STATUS",
        }
    }

    pub(crate) fn try_from_u8(value: u8) -> Result<Self> {
        let me = match value {
            0 => OpCode::Query,
            1 => OpCode::IQuery,
            2 => OpCode::Status,
            _ => return Err(Error::UnknownOpCode(value.into())),
        };
        Ok(me)
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_u8() {
        for opcode in OpCode::VALUES {
            assert_eq!(opcode, OpCode::try_from_u8(opcode as u8).unwrap());
        }

        assert!(matches!(
            OpCode::try_from_u8(128),
            Err(Error::UnknownOpCode(ov)) if ov == 128
        ));
    }

    #[test]
    fn test_values() {
        let mut count = 0;

        for opcode in OpCode::VALUES {
            let found = match opcode {
                OpCode::Query => true,
                OpCode::IQuery => true,
                OpCode::Status => true,
            };

            if found {
                count += 1;
            }
        }

        assert_eq!(count, OpCode::VALUES.len());
    }
}
