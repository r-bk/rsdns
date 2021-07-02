use crate::{constants::QType, Error, ProtocolError};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

/// Record type.
///
/// This enumeration includes data types only.
/// For data and query types see [QType].
///
/// - [RFC 1035 ~3.2.2](https://tools.ietf.org/html/rfc1035)
/// - [RFC 3596 ~2.1](https://tools.ietf.org/html/rfc3596#section-2.1) `(AAAA)`
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, EnumIter, EnumString, IntoStaticStr, Hash,
)]
pub enum RType {
    /// a host address (IPv4)
    A = 1,
    /// an authoritative name server
    Ns = 2,
    /// a mail destination (obsolete - use MX)
    Md = 3,
    /// a mail forwarder (obsolete - use MX)
    Mf = 4,
    /// the canonical name of an alias
    Cname = 5,
    /// marks the start of a zone authority
    Soa = 6,
    /// a mailbox domain name
    Mb = 7,
    /// a mail group member
    Mg = 8,
    /// a mail rename domain name
    Mr = 9,
    /// a NULL RR
    Null = 10,
    /// a well known service description
    Wks = 11,
    /// a domain name pointer
    Ptr = 12,
    /// host information
    Hinfo = 13,
    /// mailbox or mail list information
    Minfo = 14,
    /// mail exchange
    Mx = 15,
    /// text strings
    Txt = 16,
    /// a host address (IPv6)
    Aaaa = 28,
}

impl RType {
    /// Converts `RType` to a static string.
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}

impl TryFrom<u16> for RType {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => RType::A,
            2 => RType::Ns,
            3 => RType::Md,
            4 => RType::Mf,
            5 => RType::Cname,
            6 => RType::Soa,
            7 => RType::Mb,
            8 => RType::Mg,
            9 => RType::Mr,
            10 => RType::Null,
            11 => RType::Wks,
            12 => RType::Ptr,
            13 => RType::Hinfo,
            14 => RType::Minfo,
            15 => RType::Mx,
            16 => RType::Txt,
            28 => RType::Aaaa,
            _ => return Err(Error::from(ProtocolError::ReservedRType(value))),
        };

        Ok(me)
    }
}

impl Display for RType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl PartialEq<QType> for RType {
    #[inline]
    fn eq(&self, other: &QType) -> bool {
        (*self as u16) == (*other as u16)
    }
}

impl PartialOrd<QType> for RType {
    #[inline]
    fn partial_cmp(&self, other: &QType) -> Option<Ordering> {
        (*self as u16).partial_cmp(&(*other as u16))
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

        assert!(matches!(
            RType::try_from(0),
            Err(Error::ProtocolError(ProtocolError::ReservedRType(0)))
        ));
    }
}
