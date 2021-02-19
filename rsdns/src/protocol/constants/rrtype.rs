use crate::RsDnsError;
use std::convert::TryFrom;
use strum_macros::EnumIter;

/// Resource record TYPE.
///
/// [RFC 1035 ~3.2.2](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum RrType {
    // rfc 1035
    /// a host address
    A = 1,
    /// an authoritative name server
    NS = 2,
    /// a mail destination (obsolete - use MX)
    MD = 3,
    /// a mail forwarder (obsolete - use MX)
    MF = 4,
    /// the canonical name of an alias
    CNAME = 5,
    /// marks the start of a zone authority
    SOA = 6,
    /// a mailbox domain name
    MB = 7,
    /// a mail group member
    MG = 8,
    /// a mail rename domain name
    MR = 9,
    /// a NULL RR
    NULL = 10,
    /// a well known service description
    WKS = 11,
    /// a domain name pointer
    PTR = 12,
    /// host information
    HINFO = 13,
    /// mailbox or mail list information
    MINFO = 14,
    /// mail exchange
    MX = 15,
    /// text strings
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
            _ => return Err(RsDnsError::UnknownRrType(value)),
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
            Err(RsDnsError::UnknownRrType(0))
        ));
    }
}
