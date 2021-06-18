use crate::constants::RCode;
use std::fmt::{self, Display, Formatter};

/// Parsed [RCode].
///
/// This is an option-like wrapper around [RCode] to account for reserved bits
/// in the protocol definition. It is written for interoperability with more updated
/// DNS protocol implementations.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ParsedRCode {
    /// a recognized [RCode]
    Some(RCode),
    /// a reserved response code still not implemented in the [RCode] enumeration
    Reserved(u16),
}

impl ParsedRCode {
    /// Checks if parsed response code is a recognized [RCode].
    pub fn is_some(self) -> bool {
        matches!(self, ParsedRCode::Some(..))
    }

    /// Checks if parsed response code is a reserved value.
    pub fn is_reserved(self) -> bool {
        matches!(self, ParsedRCode::Reserved(..))
    }

    /// Unwraps the [RCode] value.
    ///
    /// # Panics
    ///
    /// Panics if the self value is not [`Some`](Self::Some).
    pub fn unwrap(self) -> RCode {
        if let Self::Some(response_code) = self {
            return response_code;
        }
        panic!("unwrap called on a reserved response code");
    }

    /// Unwraps the reserved [u16] value.
    ///
    /// # Panics
    ///
    /// Panics if the self value is not [`Reserved`](Self::Reserved).
    pub fn unwrap_reserved(self) -> u16 {
        if let Self::Reserved(response_code) = self {
            return response_code;
        }
        panic!("unwrap_reserved called on a recognized response code");
    }

    /// Converts parsed opcode to a string slice.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Some(response_code) => response_code.to_str(),
            Self::Reserved(bits) => Self::reserved_as_str(bits),
        }
    }

    fn reserved_as_str(bits: u16) -> &'static str {
        match bits {
            6 => "RCODE(6)",
            7 => "RCODE(7)",
            8 => "RCODE(8)",
            9 => "RCODE(9)",
            10 => "RCODE(10)",
            11 => "RCODE(11)",
            12 => "RCODE(12)",
            13 => "RCODE(13)",
            14 => "RCODE(14)",
            15 => "RCODE(15)",
            _ => "BAD_RCODE",
        }
    }
}

impl Display for ParsedRCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
