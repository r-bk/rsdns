use crate::{message::TypeValue, Error, Result};
use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

/// Record types.
///
/// - [RFC 1035 section 3.2.2](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.2)
/// - [RFC 3596 section 2.1](https://www.rfc-editor.org/rfc/rfc3596.html#section-2.1) `(AAAA)`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Type {
    /// a host address (IPv4)
    A = 1,
    /// an authoritative name server
    Ns = 2,
    /// a mail destination (obsolete - use [`Type::Mx`])
    Md = 3,
    /// a mail forwarder (obsolete - use [`Type::Mx`])
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
    /// a request for mail agent RRs (Obsolete - see [`Type::Mx`])
    Maila = 254,
    /// a request for all records
    Any = 255,
}

impl Type {
    /// Array of all discriminants in this enum.
    #[cfg(test)]
    pub const VALUES: [Type; 21] = [
        Type::A,
        Type::Ns,
        Type::Md,
        Type::Mf,
        Type::Cname,
        Type::Soa,
        Type::Mb,
        Type::Mg,
        Type::Mr,
        Type::Null,
        Type::Wks,
        Type::Ptr,
        Type::Hinfo,
        Type::Minfo,
        Type::Mx,
        Type::Txt,
        Type::Aaaa,
        Type::Axfr,
        Type::Mailb,
        Type::Maila,
        Type::Any,
    ];

    /// Converts `self` to a static string.
    #[inline]
    pub fn to_str(self) -> &'static str {
        match self {
            Type::A => "A",
            Type::Ns => "NS",
            Type::Md => "MD",
            Type::Mf => "MF",
            Type::Cname => "CNAME",
            Type::Soa => "SOA",
            Type::Mb => "MB",
            Type::Mg => "MG",
            Type::Mr => "MR",
            Type::Null => "NULL",
            Type::Wks => "WKS",
            Type::Ptr => "PTR",
            Type::Hinfo => "HINFO",
            Type::Minfo => "MINFO",
            Type::Mx => "MX",
            Type::Txt => "TXT",
            Type::Aaaa => "AAAA",
            Type::Axfr => "AXFR",
            Type::Mailb => "MAILB",
            Type::Maila => "MAILA",
            Type::Any => "ANY",
        }
    }

    pub(crate) fn try_from_u16(value: u16) -> Result<Self> {
        let me = match value {
            1 => Type::A,
            2 => Type::Ns,
            3 => Type::Md,
            4 => Type::Mf,
            5 => Type::Cname,
            6 => Type::Soa,
            7 => Type::Mb,
            8 => Type::Mg,
            9 => Type::Mr,
            10 => Type::Null,
            11 => Type::Wks,
            12 => Type::Ptr,
            13 => Type::Hinfo,
            14 => Type::Minfo,
            15 => Type::Mx,
            16 => Type::Txt,
            28 => Type::Aaaa,
            252 => Type::Axfr,
            253 => Type::Mailb,
            254 => Type::Maila,
            255 => Type::Any,
            _ => {
                return Err(Error::UnknownType(value.into()));
            }
        };

        Ok(me)
    }

    /// Checks if this is a data-type.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    #[inline]
    pub fn is_data_type(self) -> bool {
        TypeValue::from(self).is_data_type()
    }

    /// Checks if this is a question or meta-type.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    #[inline]
    pub fn is_meta_type(self) -> bool {
        TypeValue::from(self).is_meta_type()
    }
}

impl FromStr for Type {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let rtype = match s {
            "A" => Type::A,
            "NS" => Type::Ns,
            "MD" => Type::Md,
            "MF" => Type::Mf,
            "CNAME" => Type::Cname,
            "SOA" => Type::Soa,
            "MB" => Type::Mb,
            "MG" => Type::Mg,
            "MR" => Type::Mr,
            "NULL" => Type::Null,
            "WKS" => Type::Wks,
            "PTR" => Type::Ptr,
            "HINFO" => Type::Hinfo,
            "MINFO" => Type::Minfo,
            "MX" => Type::Mx,
            "TXT" => Type::Txt,
            "AAAA" => Type::Aaaa,
            "AXFR" => Type::Axfr,
            "MAILB" => Type::Mailb,
            "MAILA" => Type::Maila,
            "ANY" => Type::Any,
            _ => return Err(Error::BadParam("unknown Type string")),
        };

        Ok(rtype)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_u16() {
        for qtype in Type::VALUES {
            assert_eq!(qtype, Type::try_from_u16(qtype as u16).unwrap());
        }

        assert!(matches!(
            Type::try_from_u16(0),
            Err(Error::UnknownType(tv)) if tv == 0
        ));
    }

    #[test]
    fn test_values() {
        let mut count = 0;

        for qtype in Type::VALUES {
            let found = match qtype {
                Type::A => true,
                Type::Ns => true,
                Type::Md => true,
                Type::Mf => true,
                Type::Cname => true,
                Type::Soa => true,
                Type::Mb => true,
                Type::Mg => true,
                Type::Mr => true,
                Type::Null => true,
                Type::Wks => true,
                Type::Ptr => true,
                Type::Hinfo => true,
                Type::Minfo => true,
                Type::Mx => true,
                Type::Txt => true,
                Type::Aaaa => true,
                Type::Axfr => true,
                Type::Mailb => true,
                Type::Maila => true,
                Type::Any => true,
            };

            if found {
                count += 1;
            }
        }

        assert_eq!(count, Type::VALUES.len());
    }
}
