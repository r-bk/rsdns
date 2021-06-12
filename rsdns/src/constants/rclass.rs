use crate::Error;
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

/// Resource record class.
///
/// This enumeration includes data classes only.
/// For enumeration of all supported ones, including query classes,
/// see [QClass](crate::constants::QClass).
///
/// [RFC 1035 ~4.1.2](https://tools.ietf.org/html/rfc1035)
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr, Hash, Ord, PartialOrd,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum RClass {
    /// the internet
    IN = 1,
    /// the CSNET class (obsolete)
    CS = 2,
    /// the CHAOS class
    CH = 3,
    /// Hesiod
    HS = 4,
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
            1 => RClass::IN,
            2 => RClass::CS,
            3 => RClass::CH,
            4 => RClass::HS,
            _ => return Err(Error::ReservedRClass(value)),
        };

        Ok(me)
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

        assert!(matches!(RClass::try_from(0), Err(Error::ReservedRClass(0))));
    }
}
