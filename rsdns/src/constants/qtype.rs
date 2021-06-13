use crate::{constants::RType, Error};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

/// Query type.
///
/// This enumeration includes data and query types.
/// For data types only see [RType].
///
/// - [RFC 1035 ~3.2.2](https://tools.ietf.org/html/rfc1035)
/// - [RFC 3596 ~2.1](https://tools.ietf.org/html/rfc3596#section-2.1) `(AAAA)`
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, EnumIter, EnumString, IntoStaticStr, Hash,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum QType {
    /// a host address (IPv4)
    A = 1,
    /// an authoritative name server
    NS = 2,
    /// a mail destination (obsolete - use [`QType::MX`])
    MD = 3,
    /// a mail forwarder (obsolete - use [`QType::MX`])
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
    /// a host address (IPv6)
    AAAA = 28,
    /// a request for a transfer of an entire zone
    AXFR = 252,
    /// a request for mailbox-related records (MB, MG or MR)
    MAILB = 253,
    /// a request for mail agent RRs (Obsolete - see [`QType::MX`])
    MAILA = 254,
    /// a request for all records
    ANY = 255,
}

impl QType {
    /// Converts `QType` to a static string.
    #[inline]
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}

impl TryFrom<u16> for QType {
    type Error = Error;

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
            28 => QType::AAAA,
            252 => QType::AXFR,
            253 => QType::MAILB,
            254 => QType::MAILA,
            255 => QType::ANY,
            _ => return Err(Error::ReservedQType(value)),
        };

        Ok(me)
    }
}

impl Display for QType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl PartialEq<RType> for QType {
    #[inline]
    fn eq(&self, other: &RType) -> bool {
        (*self as u16) == (*other as u16)
    }
}

impl PartialOrd<RType> for QType {
    #[inline]
    fn partial_cmp(&self, other: &RType) -> Option<Ordering> {
        (*self as u16).partial_cmp(&(*other as u16))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::RType;
    use std::str::FromStr;
    use strum::IntoEnumIterator;

    #[test]
    fn test_try_from() {
        for qtype in QType::iter() {
            assert_eq!(qtype, QType::try_from(qtype as u16).unwrap());
        }

        assert!(matches!(QType::try_from(0), Err(Error::ReservedQType(0))));
    }

    #[test]
    fn test_rtype_compatibility() {
        for qtype in QType::iter() {
            match qtype {
                QType::AXFR | QType::MAILB | QType::MAILA | QType::ANY => continue,
                _ => {
                    assert_eq!(
                        qtype as u16,
                        RType::from_str(qtype.to_str()).unwrap() as u16
                    );
                }
            }
        }
    }
}
