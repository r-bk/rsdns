use crate::{constants::RType, Error, ProtocolError, Result};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Query type.
///
/// This enumeration includes data and query types.
/// For data types only see [RType].
///
/// - [RFC 1035 ~3.2.2](https://tools.ietf.org/html/rfc1035)
/// - [RFC 3596 ~2.1](https://tools.ietf.org/html/rfc3596#section-2.1) `(AAAA)`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QType {
    /// a host address (IPv4)
    A = 1,
    /// an authoritative name server
    Ns = 2,
    /// a mail destination (obsolete - use [`QType::Mx`])
    Md = 3,
    /// a mail forwarder (obsolete - use [`QType::Mx`])
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
    /// a request for a transfer of an entire zone
    Axfr = 252,
    /// a request for mailbox-related records (MB, MG or MR)
    Mailb = 253,
    /// a request for mail agent RRs (Obsolete - see [`QType::Mx`])
    Maila = 254,
    /// a request for all records
    Any = 255,
}

impl QType {
    /// Array of all discriminants in this enum.
    pub const VALUES: [QType; 21] = [
        QType::A,
        QType::Ns,
        QType::Md,
        QType::Mf,
        QType::Cname,
        QType::Soa,
        QType::Mb,
        QType::Mg,
        QType::Mr,
        QType::Null,
        QType::Wks,
        QType::Ptr,
        QType::Hinfo,
        QType::Minfo,
        QType::Mx,
        QType::Txt,
        QType::Aaaa,
        QType::Axfr,
        QType::Mailb,
        QType::Maila,
        QType::Any,
    ];

    /// Converts `QType` to a static string.
    #[inline]
    pub fn to_str(self) -> &'static str {
        match self {
            QType::A => "A",
            QType::Ns => "NS",
            QType::Md => "MD",
            QType::Mf => "MF",
            QType::Cname => "CNAME",
            QType::Soa => "SOA",
            QType::Mb => "MB",
            QType::Mg => "MG",
            QType::Mr => "MR",
            QType::Null => "NULL",
            QType::Wks => "WKS",
            QType::Ptr => "PTR",
            QType::Hinfo => "HINFO",
            QType::Minfo => "MINFO",
            QType::Mx => "MX",
            QType::Txt => "TXT",
            QType::Aaaa => "AAAA",
            QType::Axfr => "AXFR",
            QType::Mailb => "MAILB",
            QType::Maila => "MAILA",
            QType::Any => "ANY",
        }
    }
}

impl TryFrom<u16> for QType {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self> {
        let me = match value {
            1 => QType::A,
            2 => QType::Ns,
            3 => QType::Md,
            4 => QType::Mf,
            5 => QType::Cname,
            6 => QType::Soa,
            7 => QType::Mb,
            8 => QType::Mg,
            9 => QType::Mr,
            10 => QType::Null,
            11 => QType::Wks,
            12 => QType::Ptr,
            13 => QType::Hinfo,
            14 => QType::Minfo,
            15 => QType::Mx,
            16 => QType::Txt,
            28 => QType::Aaaa,
            252 => QType::Axfr,
            253 => QType::Mailb,
            254 => QType::Maila,
            255 => QType::Any,
            _ => return Err(Error::from(ProtocolError::ReservedQType(value))),
        };

        Ok(me)
    }
}

impl FromStr for QType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let qtype = match s {
            "A" => QType::A,
            "NS" => QType::Ns,
            "MD" => QType::Md,
            "MF" => QType::Mf,
            "CNAME" => QType::Cname,
            "SOA" => QType::Soa,
            "MB" => QType::Mb,
            "MG" => QType::Mg,
            "MR" => QType::Mr,
            "NULL" => QType::Null,
            "WKS" => QType::Wks,
            "PTR" => QType::Ptr,
            "HINFO" => QType::Hinfo,
            "MINFO" => QType::Minfo,
            "MX" => QType::Mx,
            "TXT" => QType::Txt,
            "AAAA" => QType::Aaaa,
            "AXFR" => QType::Axfr,
            "MAILB" => QType::Mailb,
            "MAILA" => QType::Maila,
            "ANY" => QType::Any,
            _ => return Err(Error::BadStr),
        };

        Ok(qtype)
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

    #[test]
    fn test_try_from() {
        for qtype in QType::VALUES {
            assert_eq!(qtype, QType::try_from(qtype as u16).unwrap());
        }

        assert!(matches!(
            QType::try_from(0),
            Err(Error::ProtocolError(ProtocolError::ReservedQType(0)))
        ));
    }

    #[test]
    fn test_rtype_compatibility() {
        for qtype in QType::VALUES {
            match qtype {
                QType::Axfr | QType::Mailb | QType::Maila | QType::Any => continue,
                _ => {
                    assert_eq!(
                        qtype as u16,
                        RType::from_str(qtype.to_str()).unwrap() as u16
                    );
                }
            }
        }
    }

    #[test]
    fn test_values() {
        let mut count = 0;

        for qtype in QType::VALUES {
            let found = match qtype {
                QType::A => true,
                QType::Ns => true,
                QType::Md => true,
                QType::Mf => true,
                QType::Cname => true,
                QType::Soa => true,
                QType::Mb => true,
                QType::Mg => true,
                QType::Mr => true,
                QType::Null => true,
                QType::Wks => true,
                QType::Ptr => true,
                QType::Hinfo => true,
                QType::Minfo => true,
                QType::Mx => true,
                QType::Txt => true,
                QType::Aaaa => true,
                QType::Axfr => true,
                QType::Mailb => true,
                QType::Maila => true,
                QType::Any => true,
            };

            if found {
                count += 1;
            }
        }

        assert_eq!(count, QType::VALUES.len());
    }
}
