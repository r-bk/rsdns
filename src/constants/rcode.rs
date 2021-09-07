use crate::{Error, Result};
use std::fmt::{self, Display, Formatter};

/// Response codes.
///
/// [RFC 1035 section 4.1.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.1)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

impl RCode {
    /// Array of all discriminants in this enum.
    #[cfg(test)]
    pub const VALUES: [RCode; 6] = [
        RCode::NoError,
        RCode::FormErr,
        RCode::ServFail,
        RCode::NxDomain,
        RCode::NotImp,
        RCode::Refused,
    ];

    /// Converts an rcode to a static string.
    pub fn to_str(self) -> &'static str {
        match self {
            RCode::NoError => "NOERROR",
            RCode::FormErr => "FORMERR",
            RCode::ServFail => "SERVFAIL",
            RCode::NxDomain => "NXDOMAIN",
            RCode::NotImp => "NOTIMP",
            RCode::Refused => "REFUSED",
        }
    }

    pub(crate) fn try_from_u16(value: u16) -> Result<Self> {
        let me = match value {
            0 => RCode::NoError,
            1 => RCode::FormErr,
            2 => RCode::ServFail,
            3 => RCode::NxDomain,
            4 => RCode::NotImp,
            5 => RCode::Refused,
            _ => return Err(Error::UnknownRCode(value.into())),
        };

        Ok(me)
    }
}

impl Display for RCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_u16() {
        for r_code in RCode::VALUES {
            assert_eq!(r_code, RCode::try_from_u16(r_code as u16).unwrap());
        }

        assert!(matches!(
            RCode::try_from_u16(128),
            Err(Error::UnknownRCode(rc)) if rc == 128
        ));
    }

    #[test]
    fn test_values() {
        let mut count = 0;

        for rcode in RCode::VALUES {
            let found = match rcode {
                RCode::NoError => true,
                RCode::FormErr => true,
                RCode::ServFail => true,
                RCode::NxDomain => true,
                RCode::NotImp => true,
                RCode::Refused => true,
            };

            if found {
                count += 1;
            }
        }

        assert_eq!(count, RCode::VALUES.len());
    }
}
