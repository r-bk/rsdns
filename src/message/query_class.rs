use crate::{
    constants::{QClass, RClass},
    message::RecordClass,
    Error,
};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Parsed query class.
///
/// This struct represents a QClass parsed from a DNS message.
/// It may include a value still not supported by the [QClass] enumeration.
///
/// Convenience methods are provided to handle both supported and not supported values.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct QueryClass {
    pub(crate) value: u16,
}

impl QueryClass {
    /// Converts the QClass to a static string slice.
    ///
    /// This is equivalent to calling `to_str` on the corresponding [QClass] value.
    /// If the value is not supported in the enum, the string `"UNRECOGNIZED_QCLASS"` is
    /// returned.
    ///
    /// For numeric representation of an unsupported value see the
    /// underlying implementation of the [Display] trait.
    pub fn to_str(self) -> &'static str {
        match QClass::try_from(self.value) {
            Ok(rt) => rt.to_str(),
            _ => "UNRECOGNIZED_QCLASS",
        }
    }
}

impl From<u16> for QueryClass {
    #[inline]
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<QClass> for QueryClass {
    #[inline]
    fn from(qc: QClass) -> Self {
        Self { value: qc as u16 }
    }
}

impl From<RClass> for QueryClass {
    #[inline]
    fn from(rc: RClass) -> Self {
        Self { value: rc as u16 }
    }
}

impl From<RecordClass> for QueryClass {
    #[inline]
    fn from(rc: RecordClass) -> Self {
        Self { value: rc.value }
    }
}

impl TryFrom<QueryClass> for QClass {
    type Error = Error;

    #[inline]
    fn try_from(qc: QueryClass) -> Result<Self, Self::Error> {
        QClass::try_from(qc.value)
    }
}

impl Display for QueryClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match QClass::try_from(self.value) {
            Ok(rc) => write!(f, "{}", rc.to_str())?,
            _ => write!(f, "QCLASS({})", self.value)?,
        }
        Ok(())
    }
}

impl PartialEq<u16> for QueryClass {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl PartialEq<QueryClass> for u16 {
    #[inline]
    fn eq(&self, other: &QueryClass) -> bool {
        *self == other.value
    }
}

impl PartialOrd<u16> for QueryClass {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialOrd<QueryClass> for u16 {
    #[inline]
    fn partial_cmp(&self, other: &QueryClass) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}

impl PartialEq<RClass> for QueryClass {
    #[inline]
    fn eq(&self, other: &RClass) -> bool {
        self.value == *other as u16
    }
}

impl PartialEq<QClass> for QueryClass {
    #[inline]
    fn eq(&self, other: &QClass) -> bool {
        self.value == *other as u16
    }
}

impl PartialEq<QueryClass> for RClass {
    #[inline]
    fn eq(&self, other: &QueryClass) -> bool {
        *self as u16 == other.value
    }
}

impl PartialEq<QueryClass> for QClass {
    #[inline]
    fn eq(&self, other: &QueryClass) -> bool {
        *self as u16 == other.value
    }
}

impl PartialOrd<RClass> for QueryClass {
    #[inline]
    fn partial_cmp(&self, other: &RClass) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<QClass> for QueryClass {
    #[inline]
    fn partial_cmp(&self, other: &QClass) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<QueryClass> for RClass {
    #[inline]
    fn partial_cmp(&self, other: &QueryClass) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.value)
    }
}

impl PartialOrd<QueryClass> for QClass {
    #[inline]
    fn partial_cmp(&self, other: &QueryClass) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.value)
    }
}

impl PartialEq<RecordClass> for QueryClass {
    #[inline]
    fn eq(&self, other: &RecordClass) -> bool {
        self.value == other.value
    }
}

impl PartialEq<QueryClass> for RecordClass {
    #[inline]
    fn eq(&self, other: &QueryClass) -> bool {
        self.value == other.value
    }
}

impl PartialOrd<RecordClass> for QueryClass {
    #[inline]
    fn partial_cmp(&self, other: &RecordClass) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<QueryClass> for RecordClass {
    #[inline]
    fn partial_cmp(&self, other: &QueryClass) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
