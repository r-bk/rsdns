use crate::{
    errors::{Error, ProtocolError, Result},
    message::RecordClass,
};
use std::fmt::{self, Display, Formatter};

/// Record classes.
///
/// - [RFC 1035 section 3.2.4](https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.4)
/// - [RFC 1035 section 3.2.5](https://datatracker.ietf.org/doc/html/rfc1035#section-3.2.5)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RClass {
    /// the internet
    In = 1,
    /// the CSNET class (obsolete)
    Cs = 2,
    /// the CHAOS class
    Ch = 3,
    /// Hesiod
    Hs = 4,
    /// any class
    Any = 255,
}

impl RClass {
    /// Array of all discriminants in this enum.
    #[cfg(test)]
    pub const VALUES: [RClass; 5] = [RClass::In, RClass::Cs, RClass::Ch, RClass::Hs, RClass::Any];

    /// Converts this `RClass` to a static string.
    pub fn to_str(self) -> &'static str {
        match self {
            RClass::In => "IN",
            RClass::Cs => "CS",
            RClass::Ch => "CH",
            RClass::Hs => "HS",
            RClass::Any => "ANY",
        }
    }

    /// Checks if this is a data-class.
    ///
    /// [RFC 6895 section 3.2](https://datatracker.ietf.org/doc/html/rfc6895#section-3.2)
    #[inline]
    pub fn is_data_class(self) -> bool {
        RecordClass::from(self).is_data_class()
    }

    /// Checks if this a question or meta-class.
    ///
    /// [RFC 6895 section 3.2](https://datatracker.ietf.org/doc/html/rfc6895#section-3.2)
    #[inline]
    pub fn is_meta_class(self) -> bool {
        RecordClass::from(self).is_meta_class()
    }

    pub(crate) fn try_from_u16(value: u16) -> Result<Self> {
        let me = match value {
            1 => RClass::In,
            2 => RClass::Cs,
            3 => RClass::Ch,
            4 => RClass::Hs,
            255 => RClass::Any,
            _ => {
                return Err(Error::ProtocolError(
                    ProtocolError::UnrecognizedRecordClass(value.into()),
                ))
            }
        };

        Ok(me)
    }
}

impl Display for RClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::RecordClass;

    #[test]
    fn test_try_from_u16() {
        for qclass in RClass::VALUES {
            assert_eq!(qclass, RClass::try_from_u16(qclass as u16).unwrap());
        }

        assert!(matches!(
            RClass::try_from_u16(0),
            Err(Error::ProtocolError(
                ProtocolError::UnrecognizedRecordClass(RecordClass { value: 0 })
            ))
        ));
    }

    #[test]
    fn test_values() {
        let mut count = 0;

        for qclass in RClass::VALUES {
            let found = match qclass {
                RClass::In => true,
                RClass::Cs => true,
                RClass::Ch => true,
                RClass::Hs => true,
                RClass::Any => true,
            };

            if found {
                count += 1;
            }
        }

        assert_eq!(count, RClass::VALUES.len());
    }
}
