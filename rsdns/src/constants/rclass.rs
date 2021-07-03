use crate::{constants::QClass, Error, ProtocolError, Result};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Record class.
///
/// This enumeration includes data classes only.
/// For enumeration of data and query classes see [QClass].
///
/// [RFC 1035 ~4.1.2](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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
    /// Array of all discriminants in this enum.
    #[cfg(test)]
    pub const VALUES: [RClass; 4] = [RClass::In, RClass::Cs, RClass::Ch, RClass::Hs];

    /// Converts `RClass` to a static string.
    pub fn to_str(self) -> &'static str {
        match self {
            RClass::In => "IN",
            RClass::Cs => "CS",
            RClass::Ch => "CH",
            RClass::Hs => "HS",
        }
    }
}

impl FromStr for RClass {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let rclass = match s {
            "IN" => RClass::In,
            "CS" => RClass::Cs,
            "CH" => RClass::Ch,
            "HS" => RClass::Hs,
            _ => return Err(Error::BadStr),
        };
        Ok(rclass)
    }
}

impl TryFrom<u16> for RClass {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self> {
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

impl TryFrom<QClass> for RClass {
    type Error = Error;

    #[inline]
    fn try_from(value: QClass) -> Result<Self> {
        Self::try_from(value as u16)
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

    #[test]
    fn test_try_from() {
        for rr_class in RClass::VALUES {
            assert_eq!(rr_class, RClass::try_from(rr_class as u16).unwrap());
        }

        assert!(matches!(
            RClass::try_from(0),
            Err(Error::ProtocolError(ProtocolError::ReservedRClass(0)))
        ));
    }

    #[test]
    fn test_eq_qclass() {
        for rclass in RClass::VALUES {
            assert_eq!(rclass, QClass::try_from(rclass as u16).unwrap());
        }
    }

    #[test]
    fn test_values() {
        let mut count = 0;

        for rclass in RClass::VALUES {
            let found = match rclass {
                RClass::In => true,
                RClass::Cs => true,
                RClass::Ch => true,
                RClass::Hs => true,
            };

            if found {
                count += 1;
            }
        }

        assert_eq!(count, RClass::VALUES.len());
    }
}
