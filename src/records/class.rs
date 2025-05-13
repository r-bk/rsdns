use crate::{
    Result,
    bytes::{Cursor, Reader},
    errors::{ClassFromStrError, UnknownClassName},
};
use core::{
    cmp::Ordering,
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

const UNKNOWN_CLASS: &str = "__UNKNOWN_CLASS__";
const RFC3597_PFX: &str = "CLASS";

#[rustfmt::skip]
static NAMES: [&str; 256] = [
    "", "IN", "CS", "CH", "HS", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "ANY",
];

#[rustfmt::skip]
static KNOWN: [u8; 256] = [
    0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
];

/// DNS record class.
///
/// This struct represents a `CLASS`[^rfc1][^rfc2] value.
///
/// [`Class`] is a newtype encapsulating a `u16`. It has associated constants
/// that define the values currently supported by `rsdns`. Additionally, it
/// supports values currently not having a defined named constant, following
/// [RFC 3597].
///
/// [`Class`] follows [RFC 3597 section 5] to display unknown values.
///
/// # Examples
///
/// ```rust
/// # use rsdns::records::Class;
/// # use std::{str::FromStr, error::Error};
/// # fn foo() -> Result<(), Box<dyn Error>> {
/// // Class can be created from a u16
/// assert_eq!(Class::from(1), Class::IN);
/// assert_eq!(Class::from(255), Class::ANY);
///
/// // Class can be created from a string
/// assert_eq!(Class::from_name("IN")?, Class::IN);
/// assert_eq!(Class::from_str("IN")?, Class::IN);
/// assert_eq!(Class::from_str("CLASS1")?, Class::IN);
/// assert_eq!(Class::from_str("CLASS200")?, Class::from(200));
///
/// // Class is comparable to u16
/// assert_eq!(Class::from(1), 1);
/// assert!(1 < Class::from(255));
///
/// // Class name is queried in constant time
/// assert_eq!(Class::IN.name(), "IN");
/// assert_eq!(Class::ANY.name(), "ANY");
/// assert_eq!(Class::from(0).name(), "__UNKNOWN_CLASS__");
///
/// // Class numerical value can be obtained as u16
/// assert_eq!(Class::IN.value(), 1);
///
/// // Display implementation follows RFC3597
/// assert_eq!(format!("{}", Class::IN), "IN");
/// assert_eq!(format!("{}", Class::from(17)), "CLASS17");
/// assert_eq!(Class::from(77).to_string(), "CLASS77");
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
///
/// [^rfc1]: [RFC 1035 section 3.2.4](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.4)
///
/// [^rfc2]: [RFC 1035 section 3.2.5](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.5)
///
/// [RFC 3597]: https://www.rfc-editor.org/rfc/rfc3597.html
/// [RFC 3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Class(u16);

impl Class {
    /// The internet class
    pub const IN: Class = Class::new(1);
    /// The CSNET class (obsolete)
    pub const CS: Class = Class::new(2);
    /// The CHAOS class
    pub const CH: Class = Class::new(3);
    /// Hesiod
    pub const HS: Class = Class::new(4);
    /// Any class (*)
    pub const ANY: Class = Class::new(255);

    #[cfg(test)]
    #[allow(missing_docs)]
    pub const VALUES: [Class; 5] = [Self::IN, Self::CS, Self::CH, Self::HS, Self::ANY];

    #[inline]
    const fn new(c: u16) -> Self {
        Self(c)
    }

    /// Returns the name of the record class in constant time.
    ///
    /// If the value doesn't have a defined named constant the string
    /// `"__UNKNOWN_CLASS__"` is returned.
    ///
    /// To convert a Class to string following [RFC3597 section 5] see
    /// [`Class::fmt`].
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Class;
    /// assert_eq!(Class::IN.name(), "IN");
    /// assert_eq!(Class::from(0).name(), "__UNKNOWN_CLASS__");
    /// ```
    ///
    /// [RFC3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    #[inline]
    pub fn name(self) -> &'static str {
        let val = self.value() as usize;
        let name = if val < NAMES.len() { NAMES[val] } else { "" };
        if name.is_empty() { UNKNOWN_CLASS } else { name }
    }

    /// Returns the Class numerical value as `u16`.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Class;
    /// assert_eq!(Class::IN.value(), 1);
    /// assert_eq!(Class::from(100).value(), 100);
    /// ```
    #[inline]
    pub const fn value(self) -> u16 {
        self.0
    }

    /// Checks if this is a data-class value.
    ///
    /// [RFC 6895 section 3.2](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.2)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Class;
    /// assert_eq!(Class::IN.is_data_class(), true);
    /// assert_eq!(Class::ANY.is_data_class(), false);
    /// assert_eq!(Class::from(u16::MAX).is_data_class(), false);
    /// ```
    #[inline]
    pub const fn is_data_class(self) -> bool {
        0x0001 <= self.0 && self.0 <= 0x007F
    }

    /// Checks if this a meta-class value.
    ///
    /// [RFC 6895 section 3.2](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.2)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Class;
    /// assert_eq!(Class::IN.is_meta_class(), false);
    /// assert_eq!(Class::ANY.is_meta_class(), true);
    /// assert_eq!(Class::from(u16::MAX).is_meta_class(), false);
    /// ```
    #[inline]
    pub const fn is_meta_class(self) -> bool {
        0x0080 <= self.0 && self.0 <= 0x00FF
    }

    /// Creates a Class from a class name.
    ///
    /// `name` is expected to be an all-capital name of a class, as returned
    /// from [`Class::name`]. Names of unknown classes (classes with no defined
    /// named constant) are not recognized. The comparison is case sensitive.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{records::Class, errors::UnknownClassName};
    /// # fn foo() -> Result<(), UnknownClassName> {
    /// assert_eq!(Class::from_name("IN")?, Class::IN);
    /// assert!(Class::from_name("UNKNOWN").is_err());
    /// assert!(Class::from_name("In").is_err());
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`UnknownClassName`] - the class name is not recognized
    pub fn from_name(name: &str) -> core::result::Result<Self, UnknownClassName> {
        match name.len() {
            2 => match name {
                "IN" => Ok(Self::IN),
                "CS" => Ok(Self::CS),
                "CH" => Ok(Self::CH),
                "HS" => Ok(Self::HS),
                _ => Err(UnknownClassName),
            },
            3 => match name {
                "ANY" => Ok(Self::ANY),
                _ => Err(UnknownClassName),
            },
            _ => Err(UnknownClassName),
        }
    }

    /// Checks if the Class value equals one of the defined named constants.
    ///
    /// This is implemented as an `O(1)` operation.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Class;
    /// assert!(Class::IN.is_defined());
    /// assert!(!Class::from(1000).is_defined());
    /// ```
    #[inline]
    pub fn is_defined(self) -> bool {
        let val = self.value() as usize;
        if val < KNOWN.len() {
            KNOWN[val] != 0
        } else {
            false
        }
    }
}

impl From<u16> for Class {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Display for Class {
    /// Formats the Class following [RFC3597 (section 5)] rules.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Class;
    /// assert_eq!(format!("{}", Class::IN), "IN");
    /// assert_eq!(format!("{}", Class::from(0)), "CLASS0");
    /// assert_eq!(Class::CH.to_string(), "CH");
    /// ```
    ///
    /// [RFC3597 (section 5)]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let val = self.value() as usize;
        let name = if val < NAMES.len() { NAMES[val] } else { "" };
        match name {
            "" => {
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "{}{}", RFC3597_PFX, self.0)?;
                f.pad(buf.as_str())
            }
            _ => f.pad(name),
        }
    }
}

impl PartialEq<u16> for Class {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Class> for u16 {
    #[inline]
    fn eq(&self, other: &Class) -> bool {
        *self == other.0
    }
}

impl PartialOrd<u16> for Class {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Class> for u16 {
    #[inline]
    fn partial_cmp(&self, other: &Class) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl Reader<Class> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> Result<Class> {
        Ok(Class::from(self.u16_be()?))
    }
}

impl Default for Class {
    #[inline]
    fn default() -> Self {
        Self::IN
    }
}

impl FromStr for Class {
    type Err = ClassFromStrError;

    /// Creates a Class from a string.
    ///
    /// The string `s` is expected to be a class name or a class display value
    /// as returned from [`Class::fmt`] for unknown classes.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{records::Class, errors::ClassFromStrError};
    /// # use core::str::FromStr;
    /// # fn foo() -> Result<(), ClassFromStrError> {
    /// assert_eq!(Class::from_str("IN")?, Class::IN);
    /// assert_eq!(Class::from_str("CLASS1")?, Class::IN);
    /// assert_eq!(Class::from_str("CLASS100")?, Class::from(100));
    ///
    /// assert!(Class::from_str("in").is_err());
    /// assert!(Class::from_str("Class100").is_err());
    /// assert!(Class::from_str("CLASS100000").is_err());  // exceeds u16::MAX
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    ///
    /// [RFC 3597]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    fn from_str(s: &str) -> core::result::Result<Self, ClassFromStrError> {
        if let Ok(c) = Self::from_name(s) {
            return Ok(c);
        }
        if let Some(sfx) = s.strip_prefix(RFC3597_PFX) {
            match sfx.parse::<u16>() {
                Ok(v) => Ok(Class::from(v)),
                _ => Err(ClassFromStrError),
            }
        } else {
            Err(ClassFromStrError)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(Class::default(), Class::IN);
    }

    #[test]
    fn test_name() {
        for (i, name) in NAMES.iter().enumerate() {
            let class = Class::from(i as u16);
            match class {
                Class::IN => assert_eq!(Class::IN.name(), *name),
                Class::CH => assert_eq!(Class::CH.name(), *name),
                Class::CS => assert_eq!(Class::CS.name(), *name),
                Class::HS => assert_eq!(Class::HS.name(), *name),
                Class::ANY => assert_eq!(Class::ANY.name(), *name),
                _ => assert_eq!(class.name(), UNKNOWN_CLASS),
            }
        }

        assert_eq!(Class::from(u16::MAX).name(), UNKNOWN_CLASS);
    }

    #[test]
    fn test_from_name() {
        assert_eq!(Class::from_name("IN").unwrap(), Class::IN);
        assert_eq!(Class::from_name("CS").unwrap(), Class::CS);
        assert_eq!(Class::from_name("CH").unwrap(), Class::CH);
        assert_eq!(Class::from_name("HS").unwrap(), Class::HS);
        assert_eq!(Class::from_name("ANY").unwrap(), Class::ANY);

        for (i, name) in NAMES.iter().enumerate() {
            if !name.is_empty() {
                assert_eq!(Class::from_name(name).unwrap(), Class::from(i as u16));
                assert!(Class::from_name(&name.to_lowercase()).is_err());
            }
        }
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Class::from_str("IN").unwrap(), Class::IN);
        assert_eq!(Class::from_str("CS").unwrap(), Class::CS);
        assert_eq!(Class::from_str("CH").unwrap(), Class::CH);
        assert_eq!(Class::from_str("HS").unwrap(), Class::HS);
        assert_eq!(Class::from_str("ANY").unwrap(), Class::ANY);

        for (i, name) in NAMES.iter().enumerate() {
            if !name.is_empty() {
                assert_eq!(Class::from_str(name).unwrap(), Class::from(i as u16));
                assert!(Class::from_str(&name.to_lowercase()).is_err());
            }
        }

        for i in 0..=u16::MAX {
            let s = format!("CLASS{i}");
            assert_eq!(Class::from_str(&s).unwrap(), Class::from(i));
            assert!(Class::from_str(&s.to_lowercase()).is_err());
        }

        assert!(Class::from_str("CLASS65536").is_err());
    }

    #[test]
    fn test_is_defined() {
        assert!(Class::IN.is_defined());
        assert!(Class::CS.is_defined());
        assert!(Class::CH.is_defined());
        assert!(Class::HS.is_defined());
        assert!(Class::ANY.is_defined());

        for (i, name) in NAMES.iter().enumerate() {
            assert_eq!(Class::from(i as u16).is_defined(), !name.is_empty());
        }

        for i in 0..=u16::MAX {
            assert_eq!(
                Class::from(i).is_defined(),
                Class::VALUES.iter().any(|v| *v == i)
            );
        }
    }
}
