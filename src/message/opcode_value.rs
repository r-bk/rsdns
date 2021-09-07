use crate::{constants::OpCode, Error};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Operation code value.
///
/// This struct represents an `OPCODE`[^rfc] value.
/// It may be a value still not supported by the [`OpCode`] enumeration.
///
/// [`OpCodeValue`] is interoperable with [`OpCode`] and [`u8`].
///
/// # Examples
///
/// ```rust
/// # use rsdns::{constants::OpCode, message::OpCodeValue, Error};
/// # use std::convert::TryFrom;
/// // OpCodeValue implements From<OpCode>
/// assert_eq!(OpCodeValue::from(OpCode::Query), OpCode::Query);
/// assert_eq!(OpCodeValue::from(OpCode::IQuery), 1);
///
/// // OpCode implements TryFrom<OpCodeValue>
/// assert_eq!(OpCode::try_from(OpCodeValue::from(2)).unwrap(), OpCode::Status);
///
/// // OpCodeValue implements From<u8>
/// assert!(matches!(OpCode::try_from(OpCodeValue::from(15)),
///                  Err(Error::UnknownOpCode(opcode)) if opcode == 15));
/// ```
///
/// [^rfc]: [RFC 1035 section 4.1.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.1)
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OpCodeValue(u8);

impl OpCodeValue {
    /// Converts `self` to a string.
    ///
    /// If the value is not supported in the [`OpCode`] enum, the string `"UNKNOWN_OPCODE"` is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rsdns::{constants::OpCode, message::OpCodeValue};
    /// assert_eq!(OpCodeValue::from(OpCode::IQuery).to_str(), "IQUERY");
    /// assert_eq!(OpCodeValue::from(15).to_str(), "UNKNOWN_OPCODE");
    /// ```
    #[inline]
    pub fn to_str(self) -> &'static str {
        match OpCode::try_from(self) {
            Ok(c) => c.to_str(),
            _ => "UNKNOWN_OPCODE",
        }
    }
}

impl From<u8> for OpCodeValue {
    #[inline]
    fn from(value: u8) -> Self {
        OpCodeValue(value)
    }
}

impl From<OpCodeValue> for u8 {
    #[inline]
    fn from(opcode: OpCodeValue) -> Self {
        opcode.0
    }
}

impl From<OpCode> for OpCodeValue {
    #[inline]
    fn from(opcode: OpCode) -> Self {
        OpCodeValue(opcode as u8)
    }
}

impl TryFrom<OpCodeValue> for OpCode {
    type Error = Error;

    #[inline]
    fn try_from(value: OpCodeValue) -> Result<Self, Self::Error> {
        OpCode::try_from_u8(value.0)
    }
}

impl Display for OpCodeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match OpCode::try_from(*self) {
            Ok(c) => f.pad(c.to_str())?,
            _ => {
                use std::fmt::Write;
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "OPCODE{}", self.0)?;
                f.pad(buf.as_str())?;
            }
        }
        Ok(())
    }
}

impl PartialEq<u8> for OpCodeValue {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialEq<OpCodeValue> for u8 {
    #[inline]
    fn eq(&self, other: &OpCodeValue) -> bool {
        *self == other.0
    }
}

impl PartialEq<OpCode> for OpCodeValue {
    #[inline]
    fn eq(&self, other: &OpCode) -> bool {
        self.0 == *other as u8
    }
}

impl PartialEq<OpCodeValue> for OpCode {
    #[inline]
    fn eq(&self, other: &OpCodeValue) -> bool {
        *self as u8 == other.0
    }
}

impl PartialOrd<OpCode> for OpCodeValue {
    #[inline]
    fn partial_cmp(&self, other: &OpCode) -> Option<Ordering> {
        self.0.partial_cmp(&(*other as u8))
    }
}

impl PartialOrd<OpCodeValue> for OpCode {
    #[inline]
    fn partial_cmp(&self, other: &OpCodeValue) -> Option<Ordering> {
        (*self as u8).partial_cmp(&other.0)
    }
}

impl PartialOrd<u8> for OpCodeValue {
    #[inline]
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<OpCodeValue> for u8 {
    #[inline]
    fn partial_cmp(&self, other: &OpCodeValue) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}
