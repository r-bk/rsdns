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
/// [`TypeValue`] is interoperable with [`Type`] and [`u16`].
///
/// [`TypeValue`] follows [RFC 3597] to display unknown values.
///
/// # Examples
///
/// ```rust
/// # use rsdns::{constants::Type, message::TypeValue, Error};
/// # use std::convert::TryFrom;
/// // TypeValue implements From<Type>
/// assert_eq!(TypeValue::from(Type::Mx), Type::Mx);
/// assert_eq!(TypeValue::from(Type::Any), 255);
///
/// // Type implements TryFrom<TypeValue>
/// assert_eq!(Type::try_from(TypeValue::from(255)).unwrap(), Type::Any);
///
/// // TypeValue implements From<u16>
/// assert!(matches!(Type::try_from(TypeValue::from(u16::MAX)),
///                  Err(Error::UnknownType(rtype)) if rtype == u16::MAX));
///
/// // Display implementation follows rfc3597
/// assert_eq!(format!("{}", TypeValue::from(Type::Txt)).as_str(), "TXT");
/// assert_eq!(format!("{}", TypeValue::from(29)).as_str(), "TYPE29");
/// ```
///
/// [^rfc]: [RFC 1035 section 3.2.2](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.2)
///
/// [RFC 3597]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct TypeValue(u16);

impl TypeValue {
    /// Converts `self` to a string.
    ///
    /// If the value is not supported in the [`Type`] enum, the string `"UNKNOWN_TYPE"` is
    /// returned.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::Type, message::TypeValue};
    /// assert_eq!(TypeValue::from(Type::Cname).to_str(), "CNAME");
    /// assert_eq!(TypeValue::from(u16::MAX).to_str(), "UNKNOWN_TYPE");
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
    /// # use rsdns::{constants::Type, message::TypeValue};
    /// assert_eq!(TypeValue::from(Type::A).is_data_type(), true);
    /// assert_eq!(TypeValue::from(Type::Any).is_data_type(), false);
    /// assert_eq!(TypeValue::from(u16::MAX).is_data_type(), false);
    /// ```
    #[inline]
    pub fn is_data_type(self) -> bool {
        (0x0001 <= self.0 && self.0 <= 0x007F) || (0x0100 <= self.0 && self.0 <= 0xEFFF)
    }

    /// Checks if this is a question-type or meta-type value.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::Type, message::TypeValue};
    /// assert_eq!(TypeValue::from(Type::Any).is_meta_type(), true);
    /// assert_eq!(TypeValue::from(Type::A).is_meta_type(), false);
    /// assert_eq!(TypeValue::from(u16::MAX).is_meta_type(), false);
    /// ```
    #[inline]
    pub fn is_meta_type(self) -> bool {
        0x0080 <= self.0 && self.0 <= 0x00FF
    }
}

impl From<u16> for TypeValue {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Type> for TypeValue {
    #[inline]
    fn from(rt: Type) -> Self {
        Self(rt as u16)
    }
}

impl TryFrom<TypeValue> for Type {
    type Error = Error;

    #[inline]
    fn try_from(rt: TypeValue) -> Result<Self> {
        Type::try_from_u16(rt.0)
    }
}

impl TryFrom<&TypeValue> for Type {
    type Error = Error;

    #[inline]
    fn try_from(rtype: &TypeValue) -> Result<Self> {
        Self::try_from_u16(rtype.0)
    }
}

impl Display for TypeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match Type::try_from(self) {
            Ok(rt) => f.pad(rt.to_str())?,
            _ => {
                use std::fmt::Write;
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "TYPE{}", self.0)?;
                f.pad(buf.as_str())?;
            }
        }
        Ok(())
    }
}

impl PartialEq<u16> for TypeValue {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<TypeValue> for u16 {
    #[inline]
    fn eq(&self, other: &TypeValue) -> bool {
        *self == other.0
    }
}

impl PartialOrd<u16> for TypeValue {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<TypeValue> for u16 {
    #[inline]
    fn partial_cmp(&self, other: &TypeValue) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl PartialEq<Type> for TypeValue {
    #[inline]
    fn eq(&self, other: &Type) -> bool {
        self.0 == *other as u16
    }
}

impl PartialEq<TypeValue> for Type {
    #[inline]
    fn eq(&self, other: &TypeValue) -> bool {
        *self as u16 == other.0
    }
}

impl PartialOrd<Type> for TypeValue {
    #[inline]
    fn partial_cmp(&self, other: &Type) -> Option<Ordering> {
        self.0.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<TypeValue> for Type {
    #[inline]
    fn partial_cmp(&self, other: &TypeValue) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.0)
    }
}

impl Reader<TypeValue> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> Result<TypeValue> {
        Ok(TypeValue::from(self.u16_be()?))
    }
}
