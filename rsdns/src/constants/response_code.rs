use crate::Error;
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};
use strum_macros::{EnumIter, EnumString, IntoStaticStr};

/// Query response code.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, EnumIter, EnumString, IntoStaticStr, Hash,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum ResponseCode {
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

impl ResponseCode {
    /// Converts `ResponseCode` to a static string.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

impl TryFrom<u8> for ResponseCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let me = match value {
            0 => ResponseCode::NOERROR,
            1 => ResponseCode::FORMERR,
            2 => ResponseCode::SERVFAIL,
            3 => ResponseCode::NXDOMAIN,
            4 => ResponseCode::NOTIMP,
            5 => ResponseCode::REFUSED,
            _ => return Err(Error::ReservedResponseCode(value)),
        };

        Ok(me)
    }
}

impl Display for ResponseCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_try_from() {
        for r_code in ResponseCode::iter() {
            assert_eq!(r_code, ResponseCode::try_from(r_code as u8).unwrap());
        }

        assert!(matches!(
            ResponseCode::try_from(128),
            Err(Error::ReservedResponseCode(128))
        ));
    }
}
