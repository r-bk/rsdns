use crate::{
    bytes::{Cursor, Reader},
    constants::Type,
    Error, Result,
};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Record type value.
///
/// This struct represents an `RTYPE`[^rfc] value.
/// It may be a value still not supported by the [`Type`] enumeration.
///
/// [`RecordType`] is interoperable with [`Type`] and [`u16`].
///
/// [`RecordType`] follows [RFC 3597] to display unknown values.
///
/// # Examples
///
/// ```rust
/// # use rsdns::{constants::Type, message::RecordType, Error};
/// # use std::convert::TryFrom;
/// // RecordType implements From<Type>
/// assert_eq!(RecordType::from(Type::Mx), Type::Mx);
/// assert_eq!(RecordType::from(Type::Any), 255);
///
/// // Type implements TryFrom<RecordType>
/// assert_eq!(Type::try_from(RecordType::from(255)).unwrap(), Type::Any);
///
/// // RecordType implements From<u16>
/// assert!(matches!(Type::try_from(RecordType::from(u16::MAX)),
///                  Err(Error::UnknownRecordType(rtype)) if rtype == u16::MAX));
///
/// // Display implementation follows rfc3597
/// assert_eq!(format!("{}", RecordType::from(29)).as_str(), "TYPE29");
/// ```
///
/// [^rfc]: [RFC 1035 section 3.2.2](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.2)
///
/// [RFC 3597]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RecordType {
    pub(crate) value: u16,
}

impl RecordType {
    /// Converts `self` to a string.
    ///
    /// If the value is not supported in the [`Type`] enum, the string `"UNKNOWN_TYPE"` is
    /// returned.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::Type, message::RecordType};
    /// assert_eq!(RecordType::from(Type::Cname).to_str(), "CNAME");
    /// assert_eq!(RecordType::from(u16::MAX).to_str(), "UNKNOWN_TYPE");
    /// ```
    pub fn to_str(self) -> &'static str {
        match Type::try_from(self) {
            Ok(rt) => rt.to_str(),
            _ => "UNKNOWN_TYPE",
        }
    }

    /// Checks if this is a data-type value.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::Type, message::RecordType};
    /// assert_eq!(RecordType::from(Type::A).is_data_type(), true);
    /// assert_eq!(RecordType::from(Type::Any).is_data_type(), false);
    /// assert_eq!(RecordType::from(u16::MAX).is_data_type(), false);
    /// ```
    #[inline]
    pub fn is_data_type(self) -> bool {
        (0x0001 <= self.value && self.value <= 0x007F)
            || (0x0100 <= self.value && self.value <= 0xEFFF)
    }

    /// Checks if this is a question-type or meta-type value.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::Type, message::RecordType};
    /// assert_eq!(RecordType::from(Type::Any).is_meta_type(), true);
    /// assert_eq!(RecordType::from(Type::A).is_meta_type(), false);
    /// assert_eq!(RecordType::from(u16::MAX).is_meta_type(), false);
    /// ```
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

impl From<Type> for RecordType {
    #[inline]
    fn from(rt: Type) -> Self {
        Self { value: rt as u16 }
    }
}

impl TryFrom<RecordType> for Type {
    type Error = Error;

    #[inline]
    fn try_from(rt: RecordType) -> Result<Self> {
        Type::try_from_u16(rt.value)
    }
}

impl TryFrom<&RecordType> for Type {
    type Error = Error;

    #[inline]
    fn try_from(rtype: &RecordType) -> Result<Self> {
        Self::try_from_u16(rtype.value)
    }
}

impl Display for RecordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match Type::try_from(self) {
            Ok(rt) => f.pad(rt.to_str())?,
            _ => {
                use std::fmt::Write;
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "TYPE{}", self.value)?;
                f.pad(buf.as_str())?;
            }
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

impl PartialEq<Type> for RecordType {
    #[inline]
    fn eq(&self, other: &Type) -> bool {
        self.value == *other as u16
    }
}

impl PartialEq<RecordType> for Type {
    #[inline]
    fn eq(&self, other: &RecordType) -> bool {
        *self as u16 == other.value
    }
}

impl PartialOrd<Type> for RecordType {
    #[inline]
    fn partial_cmp(&self, other: &Type) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<RecordType> for Type {
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
