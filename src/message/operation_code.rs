use crate::{constants::OpCode, Error};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Operation code value.
///
/// This struct represents an `OPCODE` value.
/// It may include a value still not supported by the [`OpCode`] enumeration.
///
/// Convenience methods are provided to handle both supported and not supported values.
///
/// [RFC 1035 section 4.1.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.1)
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OperationCode {
    pub(crate) value: u8,
}

impl OperationCode {
    /// Converts this [`OperationCode`] to a static string slice.
    ///
    /// This is equivalent to calling `to_str` on the corresponding [`OpCode`] value.
    /// If the value is not supported in the enum, the string `"UNRECOGNIZED_OPCODE"` is
    /// returned.
    ///
    /// For numeric representation of an unsupported value see the implementation of the
    /// [`Display`] trait.
    #[inline]
    pub fn to_str(self) -> &'static str {
        match OpCode::try_from(self) {
            Ok(c) => c.to_str(),
            _ => "UNRECOGNIZED_OPCODE",
        }
    }
}

impl From<u8> for OperationCode {
    #[inline]
    fn from(value: u8) -> Self {
        OperationCode { value }
    }
}

impl From<OperationCode> for u8 {
    #[inline]
    fn from(opcode: OperationCode) -> Self {
        opcode.value
    }
}

impl From<OpCode> for OperationCode {
    #[inline]
    fn from(opcode: OpCode) -> Self {
        OperationCode {
            value: opcode as u8,
        }
    }
}

impl TryFrom<OperationCode> for OpCode {
    type Error = Error;

    #[inline]
    fn try_from(value: OperationCode) -> Result<Self, Self::Error> {
        OpCode::try_from_u8(value.value)
    }
}

impl Display for OperationCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match OpCode::try_from(*self) {
            Ok(c) => write!(f, "{}", c.to_str())?,
            _ => write!(f, "OPCODE({})", self.value)?,
        }
        Ok(())
    }
}

impl PartialEq<u8> for OperationCode {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        self.value == *other
    }
}

impl PartialEq<OperationCode> for u8 {
    #[inline]
    fn eq(&self, other: &OperationCode) -> bool {
        *self == other.value
    }
}

impl PartialEq<OpCode> for OperationCode {
    #[inline]
    fn eq(&self, other: &OpCode) -> bool {
        self.value == *other as u8
    }
}

impl PartialEq<OperationCode> for OpCode {
    #[inline]
    fn eq(&self, other: &OperationCode) -> bool {
        *self as u8 == other.value
    }
}

impl PartialOrd<OpCode> for OperationCode {
    #[inline]
    fn partial_cmp(&self, other: &OpCode) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u8))
    }
}

impl PartialOrd<OperationCode> for OpCode {
    #[inline]
    fn partial_cmp(&self, other: &OperationCode) -> Option<Ordering> {
        (*self as u8).partial_cmp(&other.value)
    }
}

impl PartialOrd<u8> for OperationCode {
    #[inline]
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialOrd<OperationCode> for u8 {
    #[inline]
    fn partial_cmp(&self, other: &OperationCode) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}
