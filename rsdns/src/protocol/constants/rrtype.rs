use crate::RsDnsError;
use std::convert::TryFrom;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum RrType {
    // rfc 1035
    A = 1,
    NS = 2,
    MD = 3,
    MF = 4,
    CNAME = 5,
    SOA = 6,
    MB = 7,
    MG = 8,
    MR = 9,
    NULL = 10,
    WKS = 11,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
}

impl TryFrom<u16> for RrType {
    type Error = RsDnsError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => RrType::A,
            2 => RrType::NS,
            3 => RrType::MD,
            4 => RrType::MF,
            5 => RrType::CNAME,
            6 => RrType::SOA,
            7 => RrType::MB,
            8 => RrType::MG,
            9 => RrType::MR,
            10 => RrType::NULL,
            11 => RrType::WKS,
            12 => RrType::PTR,
            13 => RrType::HINFO,
            14 => RrType::MINFO,
            15 => RrType::MX,
            16 => RrType::TXT,
            _ => return Err(RsDnsError::ProtocolUnknownRrType(value)),
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
        for rr_type in RrType::iter() {
            assert_eq!(rr_type, RrType::try_from(rr_type as u16).unwrap());
        }

        assert!(matches!(
            RrType::try_from(0),
            Err(RsDnsError::ProtocolUnknownRrType(0))
        ));
    }
}
