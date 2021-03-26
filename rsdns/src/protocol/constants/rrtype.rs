use crate::Error;
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
}

impl TryFrom<u16> for RrType {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let me = match value {
            1 => RrType::A,
            2 => RrType::Ns,
            3 => RrType::Md,
            4 => RrType::Mf,
            5 => RrType::Cname,
            6 => RrType::Soa,
            7 => RrType::Mb,
            8 => RrType::Mg,
            9 => RrType::Mr,
            10 => RrType::Null,
            11 => RrType::Wks,
            12 => RrType::Ptr,
            13 => RrType::Hinfo,
            14 => RrType::Minfo,
            15 => RrType::Mx,
            16 => RrType::Txt,
            _ => return Err(Error::UnknownRrType(value)),
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

        assert!(matches!(RrType::try_from(0), Err(Error::UnknownRrType(0))));
    }
}
