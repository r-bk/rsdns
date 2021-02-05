use crate::RsDnsError;
use std::convert::TryFrom;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum QType {
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
    AXFR = 252,
    MAILB = 253,
    MAILA = 254,
    ANY = 255,
}

impl TryFrom<u16> for QType {
    type Error = RsDnsError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => QType::A,
            2 => QType::NS,
            3 => QType::MD,
            4 => QType::MF,
            5 => QType::CNAME,
            6 => QType::SOA,
            7 => QType::MB,
            8 => QType::MG,
            9 => QType::MR,
            10 => QType::NULL,
            11 => QType::WKS,
            12 => QType::PTR,
            13 => QType::HINFO,
            14 => QType::MINFO,
            15 => QType::MX,
            16 => QType::TXT,
            252 => QType::AXFR,
            253 => QType::MAILB,
            254 => QType::MAILA,
            255 => QType::ANY,
            _ => return Err(RsDnsError::ProtocolUnknownQType(value)),
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
        for qtype in QType::iter() {
            assert_eq!(qtype, QType::try_from(qtype as u16).unwrap());
        }

        assert!(matches!(
            QType::try_from(0),
            Err(RsDnsError::ProtocolUnknownQType(0))
        ));
    }
}
