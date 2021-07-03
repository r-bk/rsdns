use crate::{constants::RClass, Error, ProtocolError};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// Query class.
///
/// This enumeration includes both data and query classes.
/// For data classes only see [RClass].
///
/// [RFC 1035 ~4.1.2](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QClass {
    /// the internet
    In = 1,
    /// the CSNET class (obsolete)
    Cs = 2,
    /// the CHAOS class
    Ch = 3,
    /// Hesiod
    Hs = 4,
    /// any class
    Any = 255,
}

impl QClass {
    /// Array of all discriminants in this enum.
    #[cfg(test)]
    pub const VALUES: [QClass; 5] = [QClass::In, QClass::Cs, QClass::Ch, QClass::Hs, QClass::Any];

    /// Converts `QClass` to a static string.
    pub fn to_str(self) -> &'static str {
        match self {
            QClass::In => "IN",
            QClass::Cs => "CS",
            QClass::Ch => "CH",
            QClass::Hs => "HS",
            QClass::Any => "ANY",
        }
    }
}

impl TryFrom<u16> for QClass {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => QClass::In,
            2 => QClass::Cs,
            3 => QClass::Ch,
            4 => QClass::Hs,
            255 => QClass::Any,
            _ => return Err(Error::from(ProtocolError::ReservedQClass(value))),
        };

        Ok(me)
    }
}

impl Display for QClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl PartialEq<RClass> for QClass {
    #[inline]
    fn eq(&self, other: &RClass) -> bool {
        (*self as u16) == (*other as u16)
    }
}

impl PartialOrd<RClass> for QClass {
    #[inline]
    fn partial_cmp(&self, other: &RClass) -> Option<Ordering> {
        (*self as u16).partial_cmp(&(*other as u16))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::RClass;
    use std::str::FromStr;

    #[test]
    fn test_try_from() {
        for qclass in QClass::VALUES {
            assert_eq!(qclass, QClass::try_from(qclass as u16).unwrap());
        }

        assert!(matches!(
            QClass::try_from(0),
            Err(Error::ProtocolError(ProtocolError::ReservedQClass(0)))
        ));
    }

    #[test]
    fn test_rclass_compatibility() {
        for qclass in QClass::VALUES {
            if qclass == QClass::Any {
                continue;
            }
            assert_eq!(
                qclass as u16,
                RClass::from_str(qclass.to_str()).unwrap() as u16
            );
            assert_eq!(qclass, RClass::try_from(qclass as u16).unwrap());
        }
    }

    #[test]
    fn test_values() {
        let mut count = 0;

        for qclass in QClass::VALUES {
            let found = match qclass {
                QClass::In => true,
                QClass::Cs => true,
                QClass::Ch => true,
                QClass::Hs => true,
                QClass::Any => true,
            };

            if found {
                count += 1;
            }
        }

        assert_eq!(count, QClass::VALUES.len());
    }
}
