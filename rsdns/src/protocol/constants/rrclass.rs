use crate::Error;
use std::convert::TryFrom;
use strum_macros::EnumIter;

/// Resource record CLASS.
///
/// [RFC 1035 ~4.1.2](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum RrClass {
    /// the internet
    IN = 1,
    /// the CSNET class (obsolete)
    CS = 2,
    /// the CHAOS class
    CH = 3,
    /// Hesiod
    HS = 4,
}

impl TryFrom<u16> for RrClass {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => RrClass::IN,
            2 => RrClass::CS,
            3 => RrClass::CH,
            4 => RrClass::HS,
            _ => return Err(Error::UnknownRrClass(value)),
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
        for rr_class in RrClass::iter() {
            assert_eq!(rr_class, RrClass::try_from(rr_class as u16).unwrap());
        }

        assert!(matches!(
            RrClass::try_from(0),
            Err(Error::UnknownRrClass(0))
        ));
    }
}
