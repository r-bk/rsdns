use crate::{constants::QClass, Error, ProtocolError};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

/// Record class.
///
/// This enumeration includes data classes only.
/// For enumeration of data and query classes see [QClass].
///
/// [RFC 1035 ~4.1.2](https://tools.ietf.org/html/rfc1035)
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr, Hash, Ord, PartialOrd,
)]
pub enum RClass {
    /// the internet
    In = 1,
    /// the CSNET class (obsolete)
    Cs = 2,
    /// the CHAOS class
    Ch = 3,
    /// Hesiod
    Hs = 4,
}

impl RClass {
    /// Converts `RClass` to a static string.
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}

impl TryFrom<u16> for RClass {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => RClass::In,
            2 => RClass::Cs,
            3 => RClass::Ch,
            4 => RClass::Hs,
            _ => return Err(Error::from(ProtocolError::ReservedRClass(value))),
        };

        Ok(me)
    }
}

impl PartialEq<QClass> for RClass {
    #[inline]
    fn eq(&self, other: &QClass) -> bool {
        (*self as u16) == (*other as u16)
    }
}

impl PartialOrd<QClass> for RClass {
    #[inline]
    fn partial_cmp(&self, other: &QClass) -> Option<Ordering> {
        (*self as u16).partial_cmp(&(*other as u16))
    }
}

impl Display for RClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_try_from() {
        for rr_class in RClass::iter() {
            assert_eq!(rr_class, RClass::try_from(rr_class as u16).unwrap());
        }

        assert!(matches!(
            RClass::try_from(0),
            Err(Error::ProtocolError(ProtocolError::ReservedRClass(0)))
        ));
    }

    #[test]
    fn test_eq_qclass() {
        for rclass in RClass::iter() {
            assert_eq!(rclass, QClass::try_from(rclass as u16).unwrap());
        }
    }
}
