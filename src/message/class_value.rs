use crate::{
    bytes::{Cursor, Reader},
    constants::Class,
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
/// It may be a value still not supported by the [`Class`] enumeration.
///
/// [`ClassValue`] is interoperable with [`Class`] and [`u16`].
///
/// [`ClassValue`] follows [RFC 3597] to display unknown values.
///
/// # Examples
///
/// ```rust
/// # use rsdns::{constants::Class, message::ClassValue, Error};
/// # use std::convert::TryFrom;
/// // ClassValue implements From<Class>
/// assert_eq!(ClassValue::from(Class::In), Class::In);
/// assert_eq!(ClassValue::from(Class::Any), 255);
///
/// // Class implements TryFrom<ClassValue>
/// assert_eq!(Class::try_from(ClassValue::from(255)).unwrap(), Class::Any);
///
/// // ClassValue implements From<u16>
/// assert!(matches!(Class::try_from(ClassValue::from(u16::MAX)),
///                  Err(Error::UnknownClass(rclass)) if rclass == u16::MAX));
///
/// // Display implementation follows rfc3597
/// assert_eq!(format!("{}", ClassValue::from(Class::In)).as_str(), "IN");
/// assert_eq!(format!("{}", ClassValue::from(17)).as_str(), "CLASS17");
/// ```
///
/// [^rfc1]: [RFC 1035 section 3.2.4](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.4)
///
/// [^rfc2]: [RFC 1035 section 3.2.5](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.5)
///
/// [RFC 3597]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct ClassValue(u16);

impl ClassValue {
    /// Converts `self` to a string.
    ///
    /// If the value is not supported in the [`Class`] enum, the string `"UNKNOWN_CLASS"` is
    /// returned.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::Class, message::ClassValue};
    /// assert_eq!(ClassValue::from(Class::In).to_str(), "IN");
    /// assert_eq!(ClassValue::from(u16::MAX).to_str(), "UNKNOWN_CLASS");
    /// ```
    pub fn to_str(self) -> &'static str {
        match Class::try_from_u16(self.0) {
            Ok(rt) => rt.to_str(),
            _ => "UNKNOWN_CLASS",
        }
    }

    /// Checks if this is a data-class value.
    ///
    /// [RFC 6895 section 3.2](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.2)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::Class, message::ClassValue};
    /// assert_eq!(ClassValue::from(Class::In).is_data_class(), true);
    /// assert_eq!(ClassValue::from(Class::Any).is_data_class(), false);
    /// assert_eq!(ClassValue::from(u16::MAX).is_data_class(), false);
    /// ```
    #[inline]
    pub fn is_data_class(self) -> bool {
        0x0001 <= self.0 && self.0 <= 0x007F
    }

    /// Checks if this a meta-class value.
    ///
    /// [RFC 6895 section 3.2](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.2)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{constants::Class, message::ClassValue};
    /// assert_eq!(ClassValue::from(Class::Any).is_meta_class(), true);
    /// assert_eq!(ClassValue::from(Class::In).is_meta_class(), false);
    /// assert_eq!(ClassValue::from(u16::MAX).is_meta_class(), false);
    /// ```
    #[inline]
    pub fn is_meta_class(self) -> bool {
        0x0080 <= self.0 && self.0 <= 0x00FF
    }
}

impl From<u16> for ClassValue {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Class> for ClassValue {
    #[inline]
    fn from(c: Class) -> Self {
        Self(c as u16)
    }
}

impl TryFrom<ClassValue> for Class {
    type Error = Error;

    #[inline]
    fn try_from(cv: ClassValue) -> Result<Self> {
        Class::try_from_u16(cv.0)
    }
}

impl TryFrom<&ClassValue> for Class {
    type Error = Error;

    #[inline]
    fn try_from(cv: &ClassValue) -> Result<Self> {
        Class::try_from_u16(cv.0)
    }
}

impl Display for ClassValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match Class::try_from_u16(self.0) {
            Ok(rc) => f.pad(rc.to_str())?,
            _ => {
                use std::fmt::Write;
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "CLASS{}", self.0)?;
                f.pad(buf.as_str())?;
            }
        }
        Ok(())
    }
}

impl PartialEq<u16> for ClassValue {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<ClassValue> for u16 {
    #[inline]
    fn eq(&self, other: &ClassValue) -> bool {
        *self == other.0
    }
}

impl PartialOrd<u16> for ClassValue {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<ClassValue> for u16 {
    #[inline]
    fn partial_cmp(&self, other: &ClassValue) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl PartialEq<Class> for ClassValue {
    #[inline]
    fn eq(&self, other: &Class) -> bool {
        self.0 == *other as u16
    }
}

impl PartialEq<ClassValue> for Class {
    #[inline]
    fn eq(&self, other: &ClassValue) -> bool {
        *self as u16 == other.0
    }
}

impl PartialOrd<Class> for ClassValue {
    #[inline]
    fn partial_cmp(&self, other: &Class) -> Option<Ordering> {
        self.0.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<ClassValue> for Class {
    #[inline]
    fn partial_cmp(&self, other: &ClassValue) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.0)
    }
}

impl Reader<ClassValue> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> Result<ClassValue> {
        Ok(ClassValue::from(self.u16_be()?))
    }
}
