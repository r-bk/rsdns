use crate::{
    constants::{QType, RType},
    Error,
};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Parsed record type.
///
/// This struct represents an RType parsed from a DNS message.
/// It may include a value still not supported by the [RType] enumeration.
///
/// Convenience methods are provided to handle both supported and not supported values.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RecordType {
    pub(crate) value: u16,
}

impl RecordType {
    /// Converts the RType to a static string slice.
    ///
    /// This is equivalent to calling `to_str` on the corresponding [RType] value.
    /// If the value is not supported in the enum, the string `"UNRECOGNIZED_RTYPE"` is
    /// returned.
    ///
    /// For numeric representation of an unsupported value see the
    /// underlying implementation of the [Display] trait.
    pub fn to_str(self) -> &'static str {
        match RType::try_from(self) {
            Ok(rt) => rt.to_str(),
            _ => "UNRECOGNIZED_RTYPE",
        }
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
    fn try_from(rt: RecordType) -> Result<Self, Self::Error> {
        RType::try_from_u16(rt.value)
    }
}

impl TryFrom<&RecordType> for RType {
    type Error = Error;

    #[inline]
    fn try_from(rtype: &RecordType) -> Result<Self, Self::Error> {
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

impl PartialEq<QType> for RecordType {
    #[inline]
    fn eq(&self, other: &QType) -> bool {
        self.value == *other as u16
    }
}

impl PartialEq<RecordType> for QType {
    #[inline]
    fn eq(&self, other: &RecordType) -> bool {
        *self as u16 == other.value
    }
}

impl PartialOrd<QType> for RecordType {
    #[inline]
    fn partial_cmp(&self, other: &QType) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<RecordType> for QType {
    #[inline]
    fn partial_cmp(&self, other: &RecordType) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.value)
    }
}
