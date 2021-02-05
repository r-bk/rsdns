use crate::RsDnsError;
use std::convert::TryFrom;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum QClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
    ANY = 255,
}

impl TryFrom<u16> for QClass {
    type Error = RsDnsError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => QClass::IN,
            2 => QClass::CS,
            3 => QClass::CH,
            4 => QClass::HS,
            255 => QClass::ANY,
            _ => return Err(RsDnsError::ProtocolUnknownQClass(value)),
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

        assert!(matches!(
            QClass::try_from(0),
            Err(RsDnsError::ProtocolUnknownQClass(0))
        ));
    }
}
