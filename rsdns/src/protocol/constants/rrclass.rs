use crate::RsDnsError;
use std::convert::TryFrom;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum RrClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl TryFrom<u16> for RrClass {
    type Error = RsDnsError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => RrClass::IN,
            2 => RrClass::CS,
            3 => RrClass::CH,
            4 => RrClass::HS,
            _ => return Err(RsDnsError::ProtocolUnknownRrClass(value)),
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
            Err(RsDnsError::ProtocolUnknownRrClass(0))
        ));
    }
}
