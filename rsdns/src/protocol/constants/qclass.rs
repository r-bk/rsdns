use crate::Error;
use std::convert::TryFrom;
use strum_macros::EnumIter;

/// DNS query CLASS.
///
/// [RFC 1035 ~4.1.2](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
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

impl TryFrom<u16> for QClass {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => QClass::In,
            2 => QClass::Cs,
            3 => QClass::Ch,
            4 => QClass::Hs,
            255 => QClass::Any,
            _ => return Err(Error::UnknownQClass(value)),
        };

        Ok(me)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_try_from() {
        for qclass in QClass::iter() {
            assert_eq!(qclass, QClass::try_from(qclass as u16).unwrap());
        }

        assert!(matches!(QClass::try_from(0), Err(Error::UnknownQClass(0))));
    }
}
