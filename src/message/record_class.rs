use crate::{
    bytes::{Cursor, Reader},
    constants::RClass,
    Error, Result,
};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Record class value.
///
/// This struct represents an `RCLASS`[^rfc1][^rfc2] value.
/// It may be a value still not supported by the [`RClass`] enumeration.
///
/// [`RecordClass`] is interoperable with [`RClass`] and [`u16`].
///
/// # Examples
///
/// ```rust
/// # use rsdns::{constants::RClass, message::RecordClass, Error};
/// # use std::convert::TryFrom;
/// assert_eq!(RecordClass::from(RClass::In), RClass::In);
/// assert_eq!(RecordClass::from(RClass::Any), 255);
/// assert_eq!(RClass::try_from(RecordClass::from(255)).unwrap(), RClass::Any);
/// assert!(matches!(RClass::try_from(RecordClass::from(u16::MAX)),
///                  Err(Error::UnrecognizedRecordClass(rclass)) if rclass == u16::MAX));
/// ```
///
/// [^rfc1]: [RFC 1035 section 3.2.4](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.4)
///
/// [^rfc2]: [RFC 1035 section 3.2.5](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.5)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RecordClass {
    pub(crate) value: u16,
}

impl RecordClass {
    /// Converts `self` to a string.
    ///
    /// If the value is not supported in the [`RClass`] enum, the string `"UNRECOGNIZED_RCLASS"` is
    /// returned.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::RClass, message::RecordClass};
    /// assert_eq!(RecordClass::from(RClass::In).to_str(), "IN");
    /// assert_eq!(RecordClass::from(u16::MAX).to_str(), "UNRECOGNIZED_RCLASS");
    /// ```
    pub fn to_str(self) -> &'static str {
        match RClass::try_from_u16(self.value) {
            Ok(rt) => rt.to_str(),
            _ => "UNRECOGNIZED_RCLASS",
        }
    }

    /// Checks if this is a data-class value.
    ///
    /// [RFC 6895 section 3.2](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.2)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::RClass, message::RecordClass};
    /// assert_eq!(RecordClass::from(RClass::In).is_data_class(), true);
    /// assert_eq!(RecordClass::from(RClass::Any).is_data_class(), false);
    /// assert_eq!(RecordClass::from(u16::MAX).is_data_class(), false);
    /// ```
    #[inline]
    pub fn is_data_class(self) -> bool {
        0x0001 <= self.value && self.value <= 0x007F
    }

    /// Checks if this a meta-class value.
    ///
    /// [RFC 6895 section 3.2](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.2)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::RClass, message::RecordClass};
    /// assert_eq!(RecordClass::from(RClass::Any).is_meta_class(), true);
    /// assert_eq!(RecordClass::from(RClass::In).is_meta_class(), false);
    /// assert_eq!(RecordClass::from(u16::MAX).is_meta_class(), false);
    /// ```
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
    fn try_from(rc: RecordClass) -> Result<Self> {
        RClass::try_from_u16(rc.value)
    }
}

impl TryFrom<&RecordClass> for RClass {
    type Error = Error;

    #[inline]
    fn try_from(rc: &RecordClass) -> Result<Self> {
        RClass::try_from_u16(rc.value)
    }
}

impl Display for RecordClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match RClass::try_from_u16(self.value) {
            Ok(rc) => f.pad(rc.to_str())?,
            _ => {
                use std::fmt::Write;
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "RCLASS({})", self.value)?;
                f.pad(buf.as_str())?;
            }
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
    fn read(&mut self) -> Result<RecordClass> {
        Ok(RecordClass::from(self.u16_be()?))
    }
}
