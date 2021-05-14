use crate::constants::OpCode;
use std::fmt::{self, Display, Formatter};

/// Parsed [OpCode].
///
/// This is an option-like wrapper around [OpCode] to account for reserved bits in the protocol
/// definition. It is written for interoperability with more updated DNS protocol implementations.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ParsedOpCode {
    /// a recognized [OpCode]
    Some(OpCode),
    /// a reserved opcode still not implemented in the [OpCode] enumeration
    Reserved(u8),
}

impl ParsedOpCode {
    /// Checks if parsed opcode is a recognized [OpCode].
    pub fn is_some(self) -> bool {
        matches!(self, ParsedOpCode::Some(..))
    }

    /// Checks if parsed opcode is a reserved value.
    pub fn is_reserved(self) -> bool {
        matches!(self, ParsedOpCode::Reserved(..))
    }

    /// Unwraps the [OpCode] value.
    ///
    /// # Panics
    ///
    /// Panics if the self value is not [`Some`](Self::Some).
    pub fn unwrap(self) -> OpCode {
        if let Self::Some(opcode) = self {
            return opcode;
        }
        panic!("unwrap called on a reserved opcode");
    }

    /// Unwraps the reserved [u8] value.
    ///
    /// # Panics
    ///
    /// Panics if the self value is not [`Reserved`](Self::Reserved).
    pub fn unwrap_reserved(self) -> u8 {
        if let Self::Reserved(opcode) = self {
            return opcode;
        }
        panic!("unwrap_reserved called on a recognized opcode");
    }

    /// Converts parsed opcode to a string slice.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Some(opcode) => opcode.as_str(),
            Self::Reserved(bits) => Self::reserved_as_str(bits),
        }
    }

    fn reserved_as_str(bits: u8) -> &'static str {
        match bits {
            3 => "OPCODE(3)",
            4 => "OPCODE(4)",
            5 => "OPCODE(5)",
            6 => "OPCODE(6)",
            7 => "OPCODE(7)",
            8 => "OPCODE(8)",
            9 => "OPCODE(9)",
            10 => "OPCODE(10)",
            11 => "OPCODE(11)",
            12 => "OPCODE(12)",
            13 => "OPCODE(13)",
            14 => "OPCODE(14)",
            15 => "OPCODE(15)",
            _ => "BAD_OPCODE",
        }
    }
}

impl Display for ParsedOpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
