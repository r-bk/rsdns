use crate::Error;
use std::convert::TryFrom;
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

/// Resource record TYPE.
///
/// [RFC 1035 ~3.2.2](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, EnumString, IntoStaticStr, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum RType {
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

impl RType {
    /// Converts `RType` to a static string.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

impl TryFrom<u16> for RType {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => RType::A,
            2 => RType::NS,
            3 => RType::MD,
            4 => RType::MF,
            5 => RType::CNAME,
            6 => RType::SOA,
            7 => RType::MB,
            8 => RType::MG,
            9 => RType::MR,
            10 => RType::NULL,
            11 => RType::WKS,
            12 => RType::PTR,
            13 => RType::HINFO,
            14 => RType::MINFO,
            15 => RType::MX,
            16 => RType::TXT,
            _ => return Err(Error::UnknownRType(value)),
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
        for rr_type in RType::iter() {
            assert_eq!(rr_type, RType::try_from(rr_type as u16).unwrap());
        }

        assert!(matches!(RType::try_from(0), Err(Error::UnknownRType(0))));
    }
}
