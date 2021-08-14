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
/// [`ResponseCode`] is interoperable with [`RCode`] and [`u16`].
///
/// # Examples
///
/// ```rust
/// # use rsdns::{constants::RCode, message::ResponseCode, Error};
/// # use std::convert::TryFrom;
/// assert_eq!(ResponseCode::from(RCode::NoError), RCode::NoError);
/// assert_eq!(ResponseCode::from(RCode::NxDomain), 3);
/// assert_eq!(RCode::try_from(ResponseCode::from(1)).unwrap(), RCode::FormErr);
/// assert!(matches!(RCode::try_from(ResponseCode::from(u16::MAX)),
///                  Err(Error::UnknownResponseCode(rcode)) if rcode == u16::MAX));
/// ```
///
/// [^rfc]: [RFC 1035 section 4.1.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.1)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct ResponseCode {
    pub(crate) value: u16,
}

impl ResponseCode {
    /// Converts `self` to a string.
    ///
    /// If the value is not supported in the [`RCode`] enum, the string `"UNKNOWN_RCODE"` is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rsdns::{constants::RCode, message::ResponseCode};
    /// assert_eq!(ResponseCode::from(RCode::NxDomain).to_str(), "NXDOMAIN");
    /// assert_eq!(ResponseCode::from(u16::MAX).to_str(), "UNKNOWN_RCODE");
    /// ```
    pub fn to_str(self) -> &'static str {
        match RCode::try_from_u16(self.value) {
            Ok(rc) => rc.to_str(),
            _ => "UNKNOWN_RCODE",
        }
    }
}

impl From<u16> for ResponseCode {
    #[inline]
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<RCode> for ResponseCode {
    #[inline]
    fn from(rc: RCode) -> Self {
        Self { value: rc as u16 }
    }
}

impl TryFrom<ResponseCode> for RCode {
    type Error = Error;

    #[inline]
    fn try_from(rc: ResponseCode) -> Result<Self, Self::Error> {
        RCode::try_from_u16(rc.value)
    }
}

impl Display for ResponseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match RCode::try_from_u16(self.value) {
            Ok(c) => f.pad(c.to_str())?,
            _ => {
                use std::fmt::Write;
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "RCODE{}", self.value)?;
                f.pad(buf.as_str())?;
            }
        }
        Ok(())
    }
}

impl PartialEq<RCode> for ResponseCode {
    #[inline]
    fn eq(&self, other: &RCode) -> bool {
        self.value == *other as u16
    }
}

impl PartialEq<ResponseCode> for RCode {
    #[inline]
    fn eq(&self, other: &ResponseCode) -> bool {
        (*self as u16) == other.value
    }
}

impl PartialOrd<RCode> for ResponseCode {
    #[inline]
    fn partial_cmp(&self, other: &RCode) -> Option<Ordering> {
        self.value.partial_cmp(&(*other as u16))
    }
}

impl PartialOrd<ResponseCode> for RCode {
    #[inline]
    fn partial_cmp(&self, other: &ResponseCode) -> Option<Ordering> {
        (*self as u16).partial_cmp(&other.value)
    }
}

impl PartialEq<u16> for ResponseCode {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl PartialEq<ResponseCode> for u16 {
    #[inline]
    fn eq(&self, other: &ResponseCode) -> bool {
        *self == other.value
    }
}

impl PartialOrd<u16> for ResponseCode {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialOrd<ResponseCode> for u16 {
    #[inline]
    fn partial_cmp(&self, other: &ResponseCode) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}
