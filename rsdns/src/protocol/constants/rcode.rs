use crate::Error;
use std::convert::TryFrom;
use strum_macros::EnumIter;

/// DNS response CODE.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum RCode {
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

impl TryFrom<u8> for RCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let me = match value {
            0 => RCode::NOERROR,
            1 => RCode::FORMERR,
            2 => RCode::SERVFAIL,
            3 => RCode::NXDOMAIN,
            4 => RCode::NOTIMP,
            5 => RCode::REFUSED,
            _ => return Err(Error::UnknownRCode(value)),
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
        for r_code in RCode::iter() {
            assert_eq!(r_code, RCode::try_from(r_code as u8).unwrap());
        }

        assert!(matches!(
            RCode::try_from(128),
            Err(Error::UnknownRCode(128))
        ));
    }
}
