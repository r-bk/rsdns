use crate::{
    bytes::{Cursor, Reader},
    constants::RType,
    Error, Result,
};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Record type value.
///
/// This struct represents a record type value.
/// It may be a value still not supported by the [`RType`] enumeration.
///
/// [RFC 1035 section 3.2.2](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.2)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RecordType {
    pub(crate) value: u16,
}

impl RecordType {
    /// Converts this [`RType`] to a static string slice.
    ///
    /// This is equivalent to calling `to_str` on the corresponding [`RType`] value.
    /// If the value is not supported in the enum, the string `"UNRECOGNIZED_RTYPE"` is
    /// returned.
    ///
    /// For numeric representation of an unsupported value see the implementation of the
    /// [`Display`] trait.
    pub fn to_str(self) -> &'static str {
        match RType::try_from(self) {
            Ok(rt) => rt.to_str(),
            _ => "UNRECOGNIZED_RTYPE",
        }
    }

    /// Checks if this is a data-type.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    #[inline]
    pub fn is_data_type(self) -> bool {
        (0x0001 <= self.value && self.value <= 0x007F)
            || (0x0100 <= self.value && self.value <= 0xEFFF)
    }

    /// Checks if this is a question or meta-type.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    #[inline]
    pub fn is_meta_type(self) -> bool {
        0x0080 <= self.value && self.value <= 0x00FF
    }
}

impl From<u16> for RecordType {
    #[inline]
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<RType> for RecordType {
    #[inline]
    fn from(rt: RType) -> Self {
        Self { value: rt as u16 }
    }
}

impl TryFrom<RecordType> for RType {
    type Error = Error;

    #[inline]
    fn try_from(rt: RecordType) -> Result<Self> {
        RType::try_from_u16(rt.value)
    }
}

impl TryFrom<&RecordType> for RType {
    type Error = Error;

    #[inline]
    fn try_from(rtype: &RecordType) -> Result<Self> {
        Self::try_from_u16(rtype.value)
    }
}

impl Display for RecordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match RType::try_from(self) {
            Ok(rt) => write!(f, "{}", rt.to_str())?,
            _ => write!(f, "RTYPE({})", self.value)?,
        }
        Ok(())
    }
}

impl PartialEq<u16> for RecordType {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl PartialEq<RecordType> for u16 {
    #[inline]
    fn eq(&self, other: &RecordType) -> bool {
        *self == other.value
    }
}

impl PartialOrd<u16> for RecordType {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialOrd<RecordType> for u16 {
    #[inline]
    fn partial_cmp(&self, other: &RecordType) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}

impl PartialEq<RType> for RecordType {
    #[inline]
    fn eq(&self, other: &RType) -> bool {
        self.value == *other as u16
    }
}

impl PartialEq<RecordType> for RType {
    #[inline]
    fn eq(&self, other: &RecordType) -> bool {
        *self as u16 == other.value
    }
}

impl PartialOrd<RType> for RecordType {
    #[inline]
    fn partial_cmp(&self, other: &RType) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<RecordType> for RType {
    #[inline]
    fn partial_cmp(&self, other: &RecordType) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.value)
    }
}

impl Reader<RecordType> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> Result<RecordType> {
        Ok(RecordType::from(self.u16_be()?))
    }
}
