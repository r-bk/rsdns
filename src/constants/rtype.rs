use crate::{
    constants::QType,
    errors::{Error, ProtocolError, Result},
};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Record type.
///
/// This enumeration includes data types only.
/// For data and query types see [QType].
///
/// - [RFC 1035 ~3.2.2](https://tools.ietf.org/html/rfc1035)
/// - [RFC 3596 ~2.1](https://tools.ietf.org/html/rfc3596#section-2.1) `(AAAA)`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
    /// Array of all discriminants in this enum.
    #[cfg(test)]
    pub const VALUES: [RType; 17] = [
        RType::A,
        RType::Ns,
        RType::Md,
        RType::Mf,
        RType::Cname,
        RType::Soa,
        RType::Mb,
        RType::Mg,
        RType::Mr,
        RType::Null,
        RType::Wks,
        RType::Ptr,
        RType::Hinfo,
        RType::Minfo,
        RType::Mx,
        RType::Txt,
        RType::Aaaa,
    ];

    /// Converts `RType` to a static string.
    pub fn to_str(self) -> &'static str {
        match self {
            RType::A => "A",
            RType::Ns => "NS",
            RType::Md => "MD",
            RType::Mf => "MF",
            RType::Cname => "CNAME",
            RType::Soa => "SOA",
            RType::Mb => "MB",
            RType::Mg => "MG",
            RType::Mr => "MR",
            RType::Null => "NULL",
            RType::Wks => "WKS",
            RType::Ptr => "PTR",
            RType::Hinfo => "HINFO",
            RType::Minfo => "MINFO",
            RType::Mx => "MX",
            RType::Txt => "TXT",
            RType::Aaaa => "AAAA",
        }
    }

    pub(crate) fn try_from_u16(value: u16) -> Result<Self> {
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
            _ => {
                return Err(Error::ProtocolError(ProtocolError::UnrecognizedRecordType(
                    value.into(),
                )))
            }
        };

        Ok(me)
    }
}

impl FromStr for RType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let rtype = match s {
            "A" => RType::A,
            "NS" => RType::Ns,
            "MD" => RType::Md,
            "MF" => RType::Mf,
            "CNAME" => RType::Cname,
            "SOA" => RType::Soa,
            "MB" => RType::Mb,
            "MG" => RType::Mg,
            "MR" => RType::Mr,
            "NULL" => RType::Null,
            "WKS" => RType::Wks,
            "PTR" => RType::Ptr,
            "HINFO" => RType::Hinfo,
            "MINFO" => RType::Minfo,
            "MX" => RType::Mx,
            "TXT" => RType::Txt,
            "AAAA" => RType::Aaaa,
            _ => return Err(Error::BadInput("unrecognized RType str")),
        };

        Ok(rtype)
    }
}

impl TryFrom<QType> for RType {
    type Error = Error;

    #[inline]
    fn try_from(value: QType) -> Result<Self> {
        Self::try_from_u16(value as u16)
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
    use crate::message::RecordType;

    #[test]
    fn test_try_from_u16() {
        for rr_type in RType::VALUES {
            assert_eq!(rr_type, RType::try_from_u16(rr_type as u16).unwrap());
        }

        assert!(matches!(
            RType::try_from_u16(0),
            Err(Error::ProtocolError(ProtocolError::UnrecognizedRecordType(
                RecordType { value: 0 }
            )))
        ));
    }

    #[test]
    fn test_values() {
        let mut count = 0;

        for rtype in RType::VALUES {
            let found = match rtype {
                RType::A => true,
                RType::Ns => true,
                RType::Md => true,
                RType::Mf => true,
                RType::Cname => true,
                RType::Soa => true,
                RType::Mb => true,
                RType::Mg => true,
                RType::Mr => true,
                RType::Null => true,
                RType::Wks => true,
                RType::Ptr => true,
                RType::Hinfo => true,
                RType::Minfo => true,
                RType::Mx => true,
                RType::Txt => true,
                RType::Aaaa => true,
            };

            if found {
                count += 1;
            }
        }

        assert_eq!(count, RType::VALUES.len());
    }
}
