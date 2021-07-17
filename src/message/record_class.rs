use crate::{
    bytes::{Cursor, Reader},
    constants::RClass,
    Error, ProtocolResult,
};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Record class value.
///
/// This struct represents a record class value.
/// It may include a value still not supported by the [RClass] enumeration.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RecordClass {
    pub(crate) value: u16,
}

impl RecordClass {
    /// Converts the RecordClass to a static string slice.
    ///
    /// This is equivalent to calling `to_str` on the corresponding [RClass] value.
    /// If the value is not supported in the enum, the string `"UNRECOGNIZED_RCLASS"` is
    /// returned.
    ///
    /// For numeric representation of an unsupported value see the
    /// underlying implementation of the [Display] trait.
    pub fn to_str(self) -> &'static str {
        match RClass::try_from_u16(self.value) {
            Ok(rt) => rt.to_str(),
            _ => "UNRECOGNIZED_RCLASS",
        }
    }

    /// Checks if this is a data-class value.
    ///
    /// [RFC 6895 section 3.2](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.2)
    #[inline]
    pub fn is_data_class(self) -> bool {
        0x0001 <= self.value && self.value <= 0x007F
    }

    /// Checks if this a meta-class value.
    ///
    /// [RFC 6895 section 3.2](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.2)
    #[inline]
    pub fn is_meta_class(self) -> bool {
        0x0080 <= self.value && self.value <= 0x00FF
    }
}

impl From<u16> for RecordClass {
    #[inline]
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<RClass> for RecordClass {
    #[inline]
    fn from(rc: RClass) -> Self {
        Self { value: rc as u16 }
    }
}

impl TryFrom<RecordClass> for RClass {
    type Error = Error;

    #[inline]
    fn try_from(rc: RecordClass) -> Result<Self, Self::Error> {
        RClass::try_from_u16(rc.value)
    }
}

impl TryFrom<&RecordClass> for RClass {
    type Error = Error;

    #[inline]
    fn try_from(rc: &RecordClass) -> Result<Self, Self::Error> {
        RClass::try_from_u16(rc.value)
    }
}

impl Display for RecordClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match RClass::try_from_u16(self.value) {
            Ok(rc) => write!(f, "{}", rc.to_str())?,
            _ => write!(f, "RCLASS({})", self.value)?,
        }
        Ok(())
    }
}

impl PartialEq<u16> for RecordClass {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl PartialEq<RecordClass> for u16 {
    #[inline]
    fn eq(&self, other: &RecordClass) -> bool {
        *self == other.value
    }
}

impl PartialOrd<u16> for RecordClass {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialOrd<RecordClass> for u16 {
    #[inline]
    fn partial_cmp(&self, other: &RecordClass) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}

impl PartialEq<RClass> for RecordClass {
    #[inline]
    fn eq(&self, other: &RClass) -> bool {
        self.value == *other as u16
    }
}

impl PartialEq<RecordClass> for RClass {
    #[inline]
    fn eq(&self, other: &RecordClass) -> bool {
        *self as u16 == other.value
    }
}

impl PartialOrd<RClass> for RecordClass {
    #[inline]
    fn partial_cmp(&self, other: &RClass) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<RecordClass> for RClass {
    #[inline]
    fn partial_cmp(&self, other: &RecordClass) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.value)
    }
}

impl Reader<RecordClass> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> ProtocolResult<RecordClass> {
        Ok(RecordClass::from(self.u16_be()?))
    }
}
