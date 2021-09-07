use crate::{message::ClassValue, Error, Result};
use std::fmt::{self, Display, Formatter};

/// Record classes.
///
/// - [RFC 1035 section 3.2.4](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.4)
/// - [RFC 1035 section 3.2.5](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.5)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Class {
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

impl Class {
    /// Array of all discriminants in this enum.
    #[cfg(test)]
    pub const VALUES: [Class; 5] = [Class::In, Class::Cs, Class::Ch, Class::Hs, Class::Any];

    /// Converts `self` to a static string.
    pub fn to_str(self) -> &'static str {
        match self {
            Class::In => "IN",
            Class::Cs => "CS",
            Class::Ch => "CH",
            Class::Hs => "HS",
            Class::Any => "ANY",
        }
    }

    /// Checks if this is a data-class.
    ///
    /// [RFC 6895 section 3.2](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.2)
    #[inline]
    pub fn is_data_class(self) -> bool {
        ClassValue::from(self).is_data_class()
    }

    /// Checks if this a question or meta-class.
    ///
    /// [RFC 6895 section 3.2](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.2)
    #[inline]
    pub fn is_meta_class(self) -> bool {
        ClassValue::from(self).is_meta_class()
    }

    pub(crate) fn try_from_u16(value: u16) -> Result<Self> {
        let me = match value {
            1 => Class::In,
            2 => Class::Cs,
            3 => Class::Ch,
            4 => Class::Hs,
            255 => Class::Any,
            _ => return Err(Error::UnknownClass(value.into())),
        };

        Ok(me)
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_u16() {
        for qclass in Class::VALUES {
            assert_eq!(qclass, Class::try_from_u16(qclass as u16).unwrap());
        }

        assert!(matches!(
            Class::try_from_u16(0),
            Err(Error::UnknownClass(cv)) if cv == 0
        ));
    }

    #[test]
    fn test_values() {
        let mut count = 0;

        for qclass in Class::VALUES {
            let found = match qclass {
                Class::In => true,
                Class::Cs => true,
                Class::Ch => true,
                Class::Hs => true,
                Class::Any => true,
            };

            if found {
                count += 1;
            }
        }

        assert_eq!(count, Class::VALUES.len());
    }
}
