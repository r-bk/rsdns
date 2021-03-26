use crate::Error;
use std::convert::TryFrom;
use strum_macros::EnumIter;

/// DNS response CODE.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter, Hash)]
pub enum RCode {
    /// No error condition
    NoError = 0,
    /// Format error - the name server was unable to interpret the query.
    FormErr = 1,
    /// Server failure - the name server was unable to process this query
    /// due to a problem with the name server.
    ServFail = 2,
    /// Name error - domain name does not exist.
    NxDomain = 3,
    /// Not implemented - the name server doesn't support the requested kind of query.
    NotImp = 4,
    /// Refused - the name server refuses to perform the specified operation for policy reasons.
    Refused = 5,
}

impl TryFrom<u8> for RCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let me = match value {
            0 => RCode::NoError,
            1 => RCode::FormErr,
            2 => RCode::ServFail,
            3 => RCode::NxDomain,
            4 => RCode::NotImp,
            5 => RCode::Refused,
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
