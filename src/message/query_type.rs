use crate::{
    constants::{QType, RType},
    message::RecordType,
    Error,
};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Parsed query type.
///
/// This struct represents a QType parsed from a DNS message.
/// It may include a value still not supported by the [QType] enumeration.
///
/// Convenience methods are provided to handle both supported and not supported values.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct QueryType {
    pub(crate) value: u16,
}

impl QueryType {
    /// Converts the QType to a static string slice.
    ///
    /// This is equivalent to calling `to_str` on the corresponding [QType] value.
    /// If the value is not supported in the enum, the string `"UNRECOGNIZED_QTYPE"` is
    /// returned.
    ///
    /// For numeric representation of an unsupported value see the
    /// underlying implementation of the [Display] trait.
    pub fn to_str(self) -> &'static str {
        match QType::try_from(self.value) {
            Ok(rt) => rt.to_str(),
            _ => "UNRECOGNIZED_QTYPE",
        }
    }
}

impl From<u16> for QueryType {
    #[inline]
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<QType> for QueryType {
    #[inline]
    fn from(qt: QType) -> Self {
        Self { value: qt as u16 }
    }
}

impl From<RType> for QueryType {
    #[inline]
    fn from(rt: RType) -> Self {
        Self { value: rt as u16 }
    }
}

impl From<RecordType> for QueryType {
    #[inline]
    fn from(rt: RecordType) -> Self {
        Self { value: rt.value }
    }
}

impl TryFrom<QueryType> for QType {
    type Error = Error;

    #[inline]
    fn try_from(qt: QueryType) -> Result<Self, Self::Error> {
        QType::try_from(qt.value)
    }
}

impl Display for QueryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match QType::try_from(self.value) {
            Ok(rt) => write!(f, "{}", rt.to_str())?,
            _ => write!(f, "QTYPE({})", self.value)?,
        }
        Ok(())
    }
}

impl PartialEq<u16> for QueryType {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl PartialEq<QueryType> for u16 {
    #[inline]
    fn eq(&self, other: &QueryType) -> bool {
        *self == other.value
    }
}

impl PartialOrd<u16> for QueryType {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialOrd<QueryType> for u16 {
    #[inline]
    fn partial_cmp(&self, other: &QueryType) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}

impl PartialEq<RType> for QueryType {
    #[inline]
    fn eq(&self, other: &RType) -> bool {
        self.value == *other as u16
    }
}

impl PartialEq<QType> for QueryType {
    #[inline]
    fn eq(&self, other: &QType) -> bool {
        self.value == *other as u16
    }
}

impl PartialEq<QueryType> for RType {
    #[inline]
    fn eq(&self, other: &QueryType) -> bool {
        *self as u16 == other.value
    }
}

impl PartialEq<QueryType> for QType {
    #[inline]
    fn eq(&self, other: &QueryType) -> bool {
        *self as u16 == other.value
    }
}

impl PartialOrd<RType> for QueryType {
    #[inline]
    fn partial_cmp(&self, other: &RType) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<QType> for QueryType {
    #[inline]
    fn partial_cmp(&self, other: &QType) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<QueryType> for RType {
    #[inline]
    fn partial_cmp(&self, other: &QueryType) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.value)
    }
}

impl PartialOrd<QueryType> for QType {
    #[inline]
    fn partial_cmp(&self, other: &QueryType) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.value)
    }
}

impl PartialEq<RecordType> for QueryType {
    #[inline]
    fn eq(&self, other: &RecordType) -> bool {
        self.value == other.value
    }
}

impl PartialEq<QueryType> for RecordType {
    #[inline]
    fn eq(&self, other: &QueryType) -> bool {
        self.value == other.value
    }
}

impl PartialOrd<RecordType> for QueryType {
    #[inline]
    fn partial_cmp(&self, other: &RecordType) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialOrd<QueryType> for RecordType {
    #[inline]
    fn partial_cmp(&self, other: &QueryType) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
