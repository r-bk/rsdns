use crate::errors::{OpCodeFromStrError, UnknownOpCodeName};
use core::{
    cmp::Ordering,
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

const UNKNOWN_OPCODE: &str = "__UNKNOWN_OPCODE__";
const RFC3597_PFX: &str = "OPCODE";

static NAMES: [&str; 7] = ["QUERY", "IQUERY", "STATUS", "", "", "", ""];

static KNOWN: [u8; 7] = [1, 1, 1, 0, 0, 0, 0];

/// DNS operation code.
///
/// This struct represents an `OPCODE`[^rfc] value.
///
/// [`OpCode`] is a newtype encapsulating a `u8`. It has associated constants
/// that define the values currently supported by `rsdns`. Additionally, it
/// supports values currently not having a defined named constant, similar to
/// requirements for TYPE and CLASS in [RFC 3597].
///
/// Although not defined for OPCODE, [`OpCode`] follows the rules defined in
/// [RFC 3597 section 5] to display unknown values.
///
/// # Examples
///
/// ```rust
/// # use rsdns::message::OpCode;
/// # use std::{str::FromStr, error::Error};
/// # fn foo() -> Result<(), Box<dyn Error>> {
/// // OpCode can be created from a u8
/// assert_eq!(OpCode::from(0), OpCode::QUERY);
/// assert_eq!(OpCode::from(2), OpCode::STATUS);
///
/// // OpCode can be created from a string
/// assert_eq!(OpCode::from_name("QUERY")?, OpCode::QUERY);
/// assert_eq!(OpCode::from_str("STATUS")?, OpCode::STATUS);
/// assert_eq!(OpCode::from_str("OPCODE1")?, OpCode::IQUERY);
/// assert_eq!(OpCode::from_str("OPCODE100")?, OpCode::from(100));
///
/// // OpCode is comparable to u8
/// assert_eq!(OpCode::from(0), 0);
/// assert!(OpCode::QUERY < 1);
///
/// // OpCode name is accessed in constant time
/// assert_eq!(OpCode::QUERY.name(), "QUERY");
/// assert_eq!(OpCode::from(20).name(), "__UNKNOWN_OPCODE__");
///
/// // OpCode numerical value can be obtained as u8
/// assert_eq!(OpCode::QUERY.value(), 0);
/// assert_eq!(OpCode::from(100).value(), 100);
///
/// // Display implementation follows RFC3597
/// assert_eq!(format!("{}", OpCode::QUERY), "QUERY");
/// assert_eq!(format!("{}", OpCode::from(100)), "OPCODE100");
/// assert_eq!(OpCode::from(2).to_string(), "STATUS");
/// assert_eq!(OpCode::from(100).to_string(), "OPCODE100");
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
///
/// [^rfc]: [RFC 1035 section 4.1.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.1)
///
/// [RFC 3597]: https://www.rfc-editor.org/rfc/rfc3597.html
/// [RFC 3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OpCode(u8);

impl OpCode {
    /// a standard query
    pub const QUERY: OpCode = OpCode::new(0);

    /// an inverse query
    pub const IQUERY: OpCode = OpCode::new(1);

    /// a server status request
    pub const STATUS: OpCode = OpCode::new(2);

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) const VALUES: [OpCode; 3] = [Self::QUERY, Self::IQUERY, Self::STATUS];

    #[inline]
    const fn new(v: u8) -> Self {
        Self(v)
    }

    /// Returns the OpCode name in constant time.
    ///
    /// If the OpCode value doesn't have a defined named constant
    /// the string `"__UNKNOWN_OPCODE__"` is returned.
    ///
    /// To convert an RCode to string following rules defined for CLASS and
    /// TYPE in [RFC3597 (section 5)] see [`OpCode::fmt`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rsdns::message::OpCode;
    /// assert_eq!(OpCode::IQUERY.name(), "IQUERY");
    /// assert_eq!(OpCode::from(15).name(), "__UNKNOWN_OPCODE__");
    /// ```
    ///
    /// [RFC3597 (section 5)]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    #[inline]
    pub fn name(self) -> &'static str {
        let val = self.value() as usize;
        let name_ = if val < NAMES.len() { NAMES[val] } else { "" };
        match name_ {
            "" => UNKNOWN_OPCODE,
            _ => name_,
        }
    }

    /// Returns the OpCode numerical value as `u8`.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::message::OpCode;
    /// assert_eq!(OpCode::STATUS.value(), 2);
    /// assert_eq!(OpCode::from(5).value(), 5);
    /// ```
    #[inline]
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Creates an OpCode from an operation code name.
    ///
    /// `name` is expected to be an all-capital name of a type,
    /// as returned from [`OpCode::name`]. Names of unknown operation codes
    /// (codes with no defined named constant) are not recognized.
    /// The comparison is case sensitive.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{message::OpCode, errors::UnknownOpCodeName};
    /// # fn foo() -> Result<(), UnknownOpCodeName> {
    /// assert_eq!(OpCode::from_name("QUERY")?, OpCode::QUERY);
    /// assert!(OpCode::from_name("UNKNOWN").is_err());
    /// assert!(OpCode::from_name("Query").is_err());
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`UnknownOpCodeName`] - the operation code name is not recognized    
    pub fn from_name(name: &str) -> core::result::Result<Self, UnknownOpCodeName> {
        match name.len() {
            5 => match name {
                "QUERY" => Ok(OpCode::QUERY),
                _ => Err(UnknownOpCodeName),
            },
            6 => match name {
                "IQUERY" => Ok(OpCode::IQUERY),
                "STATUS" => Ok(OpCode::STATUS),
                _ => Err(UnknownOpCodeName),
            },
            _ => Err(UnknownOpCodeName),
        }
    }

    /// Checks if the OpCode value equals one of the defined named constants.
    ///
    /// This is implemented as an `O(1)` operation.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::message::OpCode;
    /// assert!(OpCode::QUERY.is_defined());
    /// assert!(!OpCode::from(100).is_defined());
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

impl From<u8> for OpCode {
    #[inline]
    fn from(value: u8) -> Self {
        OpCode(value)
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = self.name();
        match name {
            UNKNOWN_OPCODE => {
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "{}{}", RFC3597_PFX, self.0)?;
                f.pad(buf.as_str())
            }
            _ => f.pad(name),
        }
    }
}

impl PartialEq<u8> for OpCode {
    #[inline]
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialEq<OpCode> for u8 {
    #[inline]
    fn eq(&self, other: &OpCode) -> bool {
        *self == other.0
    }
}

impl PartialOrd<u8> for OpCode {
    #[inline]
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<OpCode> for u8 {
    #[inline]
    fn partial_cmp(&self, other: &OpCode) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl FromStr for OpCode {
    type Err = OpCodeFromStrError;

    /// Creates an OpCode from a string.
    ///
    /// The string `s` is expected to be a code name or a code display value
    /// as returned from [`OpCode::fmt`] for unknown codes.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{message::OpCode, errors::OpCodeFromStrError};
    /// # use core::str::FromStr;
    /// # fn foo() -> Result<(), OpCodeFromStrError> {
    /// assert_eq!(OpCode::from_str("QUERY")?, OpCode::QUERY);
    /// assert_eq!(OpCode::from_str("OPCODE2")?, OpCode::STATUS);
    /// assert_eq!(OpCode::from_str("OPCODE100")?, OpCode::from(100));
    ///
    /// assert!(OpCode::from_str("Query").is_err());
    /// assert!(OpCode::from_str("Opcode2").is_err());
    /// assert!(OpCode::from_str("OPCODE256").is_err());  // exceeds u8::MAX
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    ///
    /// [RFC 3597]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(oc) = Self::from_name(s) {
            return Ok(oc);
        }
        if let Some(sfx) = s.strip_prefix(RFC3597_PFX) {
            match sfx.parse::<u8>() {
                Ok(v) => Ok(OpCode::from(v)),
                _ => Err(OpCodeFromStrError),
            }
        } else {
            Err(OpCodeFromStrError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(OpCode::QUERY.name(), "QUERY");
        assert_eq!(OpCode::IQUERY.name(), "IQUERY");
        assert_eq!(OpCode::STATUS.name(), "STATUS");

        for (i, name) in NAMES.iter().enumerate() {
            if !name.is_empty() {
                assert_eq!(OpCode::from(i as u8).name(), *name);
            } else {
                assert_eq!(OpCode::from(i as u8).name(), "__UNKNOWN_OPCODE__");
            }
        }
    }

    #[test]
    fn test_from_name() {
        assert_eq!(OpCode::from_name("QUERY").unwrap(), OpCode::QUERY);
        assert_eq!(OpCode::from_name("IQUERY").unwrap(), OpCode::IQUERY);
        assert_eq!(OpCode::from_name("STATUS").unwrap(), OpCode::STATUS);

        for (i, name) in NAMES.iter().enumerate() {
            let opcode = OpCode::from(i as u8);
            match opcode {
                OpCode::QUERY => assert_eq!(OpCode::from_name(name).unwrap(), OpCode::QUERY),
                OpCode::IQUERY => assert_eq!(OpCode::from_name(name).unwrap(), OpCode::IQUERY),
                OpCode::STATUS => assert_eq!(OpCode::from_name(name).unwrap(), OpCode::STATUS),
                _ => assert!(OpCode::from_name(name).is_err()),
            }
            assert!(OpCode::from_name(&name.to_lowercase()).is_err());
        }
    }

    #[test]
    fn test_from_str() {
        assert_eq!(OpCode::from_str("QUERY").unwrap(), OpCode::QUERY);
        assert_eq!(OpCode::from_str("IQUERY").unwrap(), OpCode::IQUERY);
        assert_eq!(OpCode::from_str("STATUS").unwrap(), OpCode::STATUS);

        for (i, name) in NAMES.iter().enumerate() {
            if !name.is_empty() {
                assert_eq!(OpCode::from_str(name).unwrap(), OpCode::from(i as u8));
                assert!(OpCode::from_str(&name.to_lowercase()).is_err());
            }
        }

        for i in 0..=u8::MAX {
            let s = format!("OPCODE{i}");
            assert_eq!(OpCode::from_str(&s).unwrap(), OpCode::from(i));
            assert!(OpCode::from_str(&s.to_lowercase()).is_err());
        }
    }

    #[test]
    fn test_is_defined() {
        assert!(OpCode::QUERY.is_defined());
        assert!(OpCode::IQUERY.is_defined());
        assert!(OpCode::STATUS.is_defined());

        for (i, name) in NAMES.iter().enumerate() {
            assert_eq!(OpCode::from(i as u8).is_defined(), !name.is_empty());
        }

        for i in 0..=255 {
            assert_eq!(
                OpCode::from(i).is_defined(),
                OpCode::VALUES.iter().any(|v| *v == i)
            );
        }
    }
}
