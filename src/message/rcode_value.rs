use crate::{constants::RCode, Error};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Response code value.
///
/// This struct represents an `RCODE`[^rfc] value.
/// It may be a value still not supported by the [`RCode`] enumeration.
///
/// [`RCodeValue`] is interoperable with [`RCode`] and [`u16`].
///
/// # Examples
///
/// ```rust
/// # use rsdns::{constants::RCode, message::RCodeValue, Error};
/// # use std::convert::TryFrom;
/// // RCodeValue implements From<RCode>
/// assert_eq!(RCodeValue::from(RCode::NoError), RCode::NoError);
/// assert_eq!(RCodeValue::from(RCode::NxDomain), 3);
///
/// // RCode implements TryFrom<RCodeValue>
/// assert_eq!(RCode::try_from(RCodeValue::from(1)).unwrap(), RCode::FormErr);
///
/// // RCodeValue implements From<u16>
/// assert!(matches!(RCode::try_from(RCodeValue::from(u16::MAX)),
///                  Err(Error::UnknownRCode(rcode)) if rcode == u16::MAX));
/// ```
///
/// [^rfc]: [RFC 1035 section 4.1.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.1)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct RCodeValue(u16);

impl RCodeValue {
    /// Converts `self` to a string.
    ///
    /// If the value is not supported in the [`RCode`] enum, the string `"UNKNOWN_RCODE"` is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rsdns::{constants::RCode, message::RCodeValue};
    /// assert_eq!(RCodeValue::from(RCode::NxDomain).to_str(), "NXDOMAIN");
    /// assert_eq!(RCodeValue::from(u16::MAX).to_str(), "UNKNOWN_RCODE");
    /// ```
    pub fn to_str(self) -> &'static str {
        match RCode::try_from_u16(self.0) {
            Ok(rc) => rc.to_str(),
            _ => "UNKNOWN_RCODE",
        }
    }
}

impl From<u16> for RCodeValue {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<RCode> for RCodeValue {
    #[inline]
    fn from(rc: RCode) -> Self {
        Self(rc as u16)
    }
}

impl TryFrom<RCodeValue> for RCode {
    type Error = Error;

    #[inline]
    fn try_from(rc: RCodeValue) -> Result<Self, Self::Error> {
        RCode::try_from_u16(rc.0)
    }
}

impl Display for RCodeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match RCode::try_from_u16(self.0) {
            Ok(c) => f.pad(c.to_str())?,
            _ => {
                use std::fmt::Write;
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "RCODE{}", self.0)?;
                f.pad(buf.as_str())?;
            }
        }
        Ok(())
    }
}

impl PartialEq<RCode> for RCodeValue {
    #[inline]
    fn eq(&self, other: &RCode) -> bool {
        self.0 == *other as u16
    }
}

impl PartialEq<RCodeValue> for RCode {
    #[inline]
    fn eq(&self, other: &RCodeValue) -> bool {
        (*self as u16) == other.0
    }
}

impl PartialOrd<RCode> for RCodeValue {
    #[inline]
    fn partial_cmp(&self, other: &RCode) -> Option<Ordering> {
        self.0.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<RCodeValue> for RCode {
    #[inline]
    fn partial_cmp(&self, other: &RCodeValue) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.0)
    }
}

impl PartialEq<u16> for RCodeValue {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<RCodeValue> for u16 {
    #[inline]
    fn eq(&self, other: &RCodeValue) -> bool {
        *self == other.0
    }
}

impl PartialOrd<u16> for RCodeValue {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<RCodeValue> for u16 {
    #[inline]
    fn partial_cmp(&self, other: &RCodeValue) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}
