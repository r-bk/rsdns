use crate::errors::{Error, ProtocolError, Result};
use crate::message::RecordType;
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Record types.
///
/// - [RFC 1035 section 3.2.2](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.2)
/// - [RFC 3596 section 2.1](https://www.rfc-editor.org/rfc/rfc3596.html#section-2.1) `(AAAA)`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RType {
    /// a host address (IPv4)
    A = 1,
    /// an authoritative name server
    Ns = 2,
    /// a mail destination (obsolete - use [`RType::Mx`])
    Md = 3,
    /// a mail forwarder (obsolete - use [`RType::Mx`])
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
    /// a request for mail agent RRs (Obsolete - see [`RType::Mx`])
    Maila = 254,
    /// a request for all records
    Any = 255,
}

impl RType {
    /// Array of all discriminants in this enum.
    #[cfg(test)]
    pub const VALUES: [RType; 21] = [
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
        RType::Axfr,
        RType::Mailb,
        RType::Maila,
        RType::Any,
    ];

    /// Converts `RType` to a static string.
    #[inline]
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
            RType::Axfr => "AXFR",
            RType::Mailb => "MAILB",
            RType::Maila => "MAILA",
            RType::Any => "ANY",
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
            252 => RType::Axfr,
            253 => RType::Mailb,
            254 => RType::Maila,
            255 => RType::Any,
            _ => {
                return Err(Error::from(ProtocolError::UnrecognizedRecordType(
                    value.into(),
                )));
            }
        };

        Ok(me)
    }

    /// Checks if this is a data-type.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    #[inline]
    pub fn is_data_type(self) -> bool {
        RecordType::from(self).is_data_type()
    }

    /// Checks if this is a question or meta-type.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    #[inline]
    pub fn is_meta_type(self) -> bool {
        RecordType::from(self).is_meta_type()
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
            "AXFR" => RType::Axfr,
            "MAILB" => RType::Mailb,
            "MAILA" => RType::Maila,
            "ANY" => RType::Any,
            _ => return Err(Error::BadInput("unrecognized RType str")),
        };

        Ok(rtype)
    }
}

impl Display for RType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::RecordType;

    #[test]
    fn test_try_from_u16() {
        for qtype in RType::VALUES {
            assert_eq!(qtype, RType::try_from_u16(qtype as u16).unwrap());
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

        for qtype in RType::VALUES {
            let found = match qtype {
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
                RType::Axfr => true,
                RType::Mailb => true,
                RType::Maila => true,
                RType::Any => true,
            };

            if found {
                count += 1;
            }
        }

        assert_eq!(count, RType::VALUES.len());
    }
}
