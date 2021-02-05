use crate::RsDnsError;
use std::convert::TryFrom;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum Rcode {
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
}

impl TryFrom<u8> for Rcode {
    type Error = RsDnsError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let me = match value {
            0 => Rcode::NOERROR,
            1 => Rcode::FORMERR,
            2 => Rcode::SERVFAIL,
            3 => Rcode::NXDOMAIN,
            4 => Rcode::NOTIMP,
            5 => Rcode::REFUSED,
            _ => return Err(RsDnsError::ProtocolUnknownRcode(value)),
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
        for r_code in Rcode::iter() {
            assert_eq!(r_code, Rcode::try_from(r_code as u8).unwrap());
        }

        assert!(matches!(
            Rcode::try_from(128),
            Err(RsDnsError::ProtocolUnknownRcode(128))
        ));
    }
}
