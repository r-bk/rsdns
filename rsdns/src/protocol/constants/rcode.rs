use crate::RsDnsError;
use std::convert::TryFrom;
use strum_macros::EnumIter;

/// DNS response CODE.
///
/// [RFC1045 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum Rcode {
    /// No error condition
    NOERROR = 0,
    /// Format error - the name server was unable to interpret the query.
    FORMERR = 1,
    /// Server failure - the name server was unable to process this query
    /// due to a problem with the name server.
    SERVFAIL = 2,
    /// Name error - domain name does not exist.
    NXDOMAIN = 3,
    /// Not implemented - the name server doesn't support the requested kind of query.
    NOTIMP = 4,
    /// Refused - the name server refuses to perform the specified operation for policy reasons.
    REFUSED = 5,
}

impl TryFrom<u8> for Rcode {
    type Error = RsDnsError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let me = match value {
            0 => Rcode::NOERROR,
            1 => Rcode::FORMERR,
            2 => Rcode::SERVFAIL,
            3 => Rcode::NXDOMAIN,
            4 => Rcode::NOTIMP,
            5 => Rcode::REFUSED,
            _ => return Err(RsDnsError::ProtocolUnknownRcode(value)),
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
        for r_code in Rcode::iter() {
            assert_eq!(r_code, Rcode::try_from(r_code as u8).unwrap());
        }

        assert!(matches!(
            Rcode::try_from(128),
            Err(RsDnsError::ProtocolUnknownRcode(128))
        ));
    }
}
