use crate::Error;
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

/// Query class.
///
/// [RFC 1035 ~4.1.2](https://tools.ietf.org/html/rfc1035)
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, EnumIter, EnumString, IntoStaticStr, Hash,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum QClass {
    /// the internet
    IN = 1,
    /// the CSNET class (obsolete)
    CS = 2,
    /// the CHAOS class
    CH = 3,
    /// Hesiod
    HS = 4,
    /// any class
    ANY = 255,
}

impl QClass {
    /// Converts `QClass` to a static string.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

impl TryFrom<u16> for QClass {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => QClass::IN,
            2 => QClass::CS,
            3 => QClass::CH,
            4 => QClass::HS,
            255 => QClass::ANY,
            _ => return Err(Error::ReservedQClass(value)),
        };

        Ok(me)
    }
}

impl Display for QClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::RClass;
    use std::str::FromStr;
    use strum::IntoEnumIterator;

    #[test]
    fn test_try_from() {
        for qclass in QClass::iter() {
            assert_eq!(qclass, QClass::try_from(qclass as u16).unwrap());
        }

        assert!(matches!(QClass::try_from(0), Err(Error::ReservedQClass(0))));
    }

    #[test]
    fn test_rclass_compatibility() {
        for qclass in QClass::iter() {
            if qclass == QClass::ANY {
                continue;
            }
            assert_eq!(
                qclass as u16,
                RClass::from_str(qclass.as_str()).unwrap() as u16
            );
        }
    }
}
