use crate::{
    constants::{QClass, RClass},
    Error,
};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Parsed record class.
///
/// This struct represents an RClass parsed from a DNS message.
/// It may include a value still not supported by the [RClass] enumeration.
///
/// Convenience methods are provided to handle both supported and not supported values.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RecordClass {
    pub(crate) value: u16,
}

impl RecordClass {
    /// Converts the RClass to a static string slice.
    ///
    /// This is equivalent to calling `to_str` on the corresponding [RClass] value.
    /// If the value is not supported in the enum, the string `"UNRECOGNIZED_RCLASS"` is
    /// returned.
    ///
    /// For numeric representation of an unsupported value see the
    /// underlying implementation of the [Display] trait.
    pub fn to_str(self) -> &'static str {
        match RClass::try_from(self.value) {
            Ok(rt) => rt.to_str(),
            _ => "UNRECOGNIZED_RCLASS",
        }
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
        RClass::try_from(rc.value)
    }
}

impl Display for RecordClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match RClass::try_from(self.value) {
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

impl PartialEq<QClass> for RecordClass {
    #[inline]
    fn eq(&self, other: &QClass) -> bool {
        self.value == *other as u16
    }
}

impl PartialEq<RecordClass> for QClass {
    #[inline]
    fn eq(&self, other: &RecordClass) -> bool {
        *self as u16 == other.value
    }
}

impl PartialOrd<QClass> for RecordClass {
    #[inline]
    fn partial_cmp(&self, other: &QClass) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<RecordClass> for QClass {
    #[inline]
    fn partial_cmp(&self, other: &RecordClass) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.value)
    }
}
