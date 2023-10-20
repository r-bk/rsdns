use crate::errors::{RCodeFromStrError, UnknownRCodeName};
use core::{
    cmp::Ordering,
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

const UNKNOWN_RCODE: &str = "__UNKNOWN_RCODE__";
const RFC3597_PFX: &str = "RCODE";

#[rustfmt::skip]
static NAMES: [&str; 17] = [
    "NOERROR",          // 0
    "FORMERR",          // 1
    "SERVFAIL",         // 2
    "NXDOMAIN",         // 3
    "NOTIMP",           // 4
    "REFUSED",          // 5
    UNKNOWN_RCODE,      // 6
    UNKNOWN_RCODE,      // 7
    UNKNOWN_RCODE,      // 8
    UNKNOWN_RCODE,      // 9
    UNKNOWN_RCODE,      // 10
    UNKNOWN_RCODE,      // 11
    UNKNOWN_RCODE,      // 12
    UNKNOWN_RCODE,      // 13
    UNKNOWN_RCODE,      // 14
    UNKNOWN_RCODE,      // 15
    "BADVERS",          // 16
];

static KNOWN: [u8; 17] = [1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];

/// DNS response code.
///
/// This struct represents an `RCODE`[^rfc] value.
///
/// [`RCode`] is a newtype encapsulating a `u16`. It has associated constants
/// that define the values currently supported by `rsdns`. Additionally, it
/// supports values currently not having a defined named constant, following
/// [RFC 3597].
///
/// Although not defined for RCODE, [`RCode`] follows the rules defined in
/// [RFC 3597 section 5] to display unknown values.
///
/// # Examples
///
/// ```rust
/// # use rsdns::message::RCode;
/// # use std::{str::FromStr, error::Error};
/// # fn foo() -> Result<(), Box<dyn Error>> {
/// // RCode can be created from a u16
/// assert_eq!(RCode::from(0), RCode::NOERROR);
/// assert_eq!(RCode::from(3), RCode::NXDOMAIN);
///
/// // RCode can be created from a string
/// assert_eq!(RCode::from_name("NOERROR")?, RCode::NOERROR);
/// assert_eq!(RCode::from_str("NXDOMAIN")?, RCode::NXDOMAIN);
/// assert_eq!(RCode::from_str("RCODE2")?, RCode::SERVFAIL);
/// assert_eq!(RCode::from_str("RCODE200")?, RCode::from(200));
///
/// // RCode numerical value can be obtained as u16
/// assert_eq!(RCode::NOERROR.value(), 0);
/// assert_eq!(RCode::from(777).value(), 777);
///
/// // RCode name is queried in constant time
/// assert_eq!(RCode::NOTIMP.name(), "NOTIMP");
/// assert_eq!(RCode::from(777).name(), "__UNKNOWN_RCODE__");
///
/// // Display implementation follows RFC3597
/// assert_eq!(format!("{}", RCode::SERVFAIL), "SERVFAIL");
/// assert_eq!(format!("{}", RCode::from(777)), "RCODE777");
/// assert_eq!(RCode::from(333).to_string(), "RCODE333");
///
/// // RCode can be compared to a u16
/// assert_eq!(RCode::FORMERR, 1);
/// assert!(RCode::FORMERR < 2);
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
///
/// [^rfc]: [RFC 1035 section 4.1.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.1)
///
/// [RFC 3597]: https://www.rfc-editor.org/rfc/rfc3597.html
/// [RFC 3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct RCode(u16);

impl RCode {
    /// No error condition
    pub const NOERROR: RCode = RCode::new(0);
    /// Format error - the name server was unable to interpret the query.
    pub const FORMERR: RCode = RCode::new(1);
    /// Server failure - the name server was unable to process this query
    /// due to a problem with the name server.
    pub const SERVFAIL: RCode = RCode::new(2);
    /// Name error - domain name does not exist.
    pub const NXDOMAIN: RCode = RCode::new(3);
    /// Not implemented - the name server doesn't support the requested kind of query.
    pub const NOTIMP: RCode = RCode::new(4);
    /// Refused - the name server refuses to perform the specified operation for policy reasons.
    pub const REFUSED: RCode = RCode::new(5);
    /// Bad version
    /// [RFC 2671 section 4.6](https://www.rfc-editor.org/rfc/rfc2671.html#section-4.6)
    pub const BADVERS: RCode = RCode::new(16);

    #[cfg(test)]
    pub const VALUES: [RCode; 7] = [
        Self::NOERROR,
        Self::FORMERR,
        Self::SERVFAIL,
        Self::NXDOMAIN,
        Self::NOTIMP,
        Self::REFUSED,
        Self::BADVERS,
    ];

    #[inline]
    const fn new(v: u16) -> Self {
        Self(v)
    }

    /// Returns the name of an RCode in constant time.
    ///
    /// If the RCode doesn't have a defined named constant the string
    /// `"__UNKNOWN_RCODE__"` is returned.
    ///
    /// To convert an RCode to string following rules defined for CLASS and
    /// TYPE in [RFC3597 section 5] see [`RCode::fmt`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rsdns::message::RCode;
    /// assert_eq!(RCode::from(3).name(), "NXDOMAIN");
    /// assert_eq!(RCode::REFUSED.name(), "REFUSED");
    /// assert_eq!(RCode::from(u16::MAX).name(), "__UNKNOWN_RCODE__");
    /// ```
    ///
    /// [RFC3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    #[inline]
    pub fn name(self) -> &'static str {
        let val = self.0 as usize;
        if val < NAMES.len() {
            NAMES[val]
        } else {
            UNKNOWN_RCODE
        }
    }

    /// Returns the RCode numerical value as `u16`.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::message::RCode;
    /// assert_eq!(RCode::NXDOMAIN.value(), 3);
    /// assert_eq!(RCode::from(100).value(), 100);
    /// ```
    #[inline]
    pub const fn value(self) -> u16 {
        self.0
    }

    /// Creates an extended `RCODE` value.
    ///
    /// An extended `RCODE` value is a 12-bit `RCODE`, whose low 4 bits are taken from the message
    /// header, and high 8 bits are taken from the `OPT` pseudo-record.
    ///
    /// # Parameters
    ///
    /// - `base` - the base `RCODE` value, as returned from [`Flags::response_code`]
    /// - `extension` - the extension value, as returned from [`Opt::rcode_extension`]
    ///
    /// [`Flags::response_code`]: crate::message::Flags::response_code
    /// [`Opt::rcode_extension`]: crate::records::Opt::rcode_extension
    #[inline]
    pub fn extended(base: RCode, extension: u8) -> RCode {
        RCode((base.0 & 0xF) | ((extension as u16) << 4))
    }

    /// Creates an RCode from a return code name.
    ///
    /// `name` is expected to be an all-capital name of a return code,
    /// as returned from [`RCode::name`]. Names of unknown return codes
    /// (codes with no defined named constant) are not recognized.
    /// The comparison is case sensitive.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{message::RCode, errors::UnknownRCodeName};
    /// # fn foo() -> Result<(), UnknownRCodeName> {
    /// assert_eq!(RCode::from_name("NOERROR")?, RCode::NOERROR);
    /// assert!(RCode::from_name("UNKNOWN").is_err());
    /// assert!(RCode::from_name("NoError").is_err());
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`UnknownRCodeName`] - the return code name is not recognized
    pub fn from_name(name: &str) -> Result<Self, UnknownRCodeName> {
        match name.len() {
            6 => match name {
                "NOTIMP" => Ok(RCode::NOTIMP),
                _ => Err(UnknownRCodeName),
            },
            7 => match name {
                "NOERROR" => Ok(RCode::NOERROR),
                "FORMERR" => Ok(RCode::FORMERR),
                "REFUSED" => Ok(RCode::REFUSED),
                "BADVERS" => Ok(RCode::BADVERS),
                _ => Err(UnknownRCodeName),
            },
            8 => match name {
                "SERVFAIL" => Ok(RCode::SERVFAIL),
                "NXDOMAIN" => Ok(RCode::NXDOMAIN),
                _ => Err(UnknownRCodeName),
            },
            _ => Err(UnknownRCodeName),
        }
    }

    /// Checks if the RCode value equals one of the defined named constants.
    ///
    /// This is implemented as an `O(1)` operation.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::message::RCode;
    /// assert!(RCode::NOERROR.is_defined());
    /// assert!(!RCode::from(1000).is_defined());
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

impl From<u16> for RCode {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Display for RCode {
    /// Formats an RCode following rules similar to [RFC3597 section 5].
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::message::RCode;
    /// let rc = RCode::from(777);
    /// assert_eq!(format!("{}", rc), "RCODE777");
    /// assert_eq!(format!("{}", RCode::NOERROR), "NOERROR");
    /// assert_eq!(rc.to_string(), "RCODE777");
    /// ```
    ///
    /// [RFC3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = self.name();
        match name {
            UNKNOWN_RCODE => {
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "{}{}", RFC3597_PFX, self.0)?;
                f.pad(buf.as_str())
            }
            _ => f.pad(name),
        }
    }
}

impl PartialEq<u16> for RCode {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialEq<RCode> for u16 {
    #[inline]
    fn eq(&self, other: &RCode) -> bool {
        *self == other.0
    }
}

impl PartialOrd<u16> for RCode {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<RCode> for u16 {
    #[inline]
    fn partial_cmp(&self, other: &RCode) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl FromStr for RCode {
    type Err = RCodeFromStrError;

    /// Creates an RCode from a string.
    ///
    /// The string `s` is expected to be an RCode name or a display value
    /// as returned from [`RCode::fmt`] for unknown codes.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{message::RCode, errors::RCodeFromStrError};
    /// # use core::str::FromStr;
    /// # fn foo() -> Result<(), RCodeFromStrError> {
    /// assert_eq!(RCode::from_str("NOERROR")?, RCode::NOERROR);
    /// assert_eq!(RCode::from_str("RCODE3")?, RCode::NXDOMAIN);
    /// assert_eq!(RCode::from_str("RCODE300")?, RCode::from(300));
    ///
    /// assert!(RCode::from_str("NoError").is_err());
    /// assert!(RCode::from_str("RCode3").is_err());
    /// assert!(RCode::from_str("RCODE65536").is_err());  // exceeds u16::MAX
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(rc) = Self::from_name(s) {
            return Ok(rc);
        }
        if let Some(sfx) = s.strip_prefix(RFC3597_PFX) {
            match sfx.parse::<u16>() {
                Ok(v) => Ok(RCode::from(v)),
                _ => Err(RCodeFromStrError),
            }
        } else {
            Err(RCodeFromStrError)
        }
    }
}

#[cfg(test)]
mod test {
    pub use super::*;

    #[test]
    fn test_name() {
        assert_eq!(RCode::NOERROR.name(), "NOERROR");
        assert_eq!(RCode::FORMERR.name(), "FORMERR");
        assert_eq!(RCode::SERVFAIL.name(), "SERVFAIL");
        assert_eq!(RCode::NXDOMAIN.name(), "NXDOMAIN");
        assert_eq!(RCode::NOTIMP.name(), "NOTIMP");
        assert_eq!(RCode::REFUSED.name(), "REFUSED");
        assert_eq!(RCode::BADVERS.name(), "BADVERS");

        for (i, v) in NAMES.iter().enumerate() {
            assert_eq!(RCode::from(i as u16).name(), *v);
        }
    }

    #[test]
    fn test_from_name() {
        assert_eq!(RCode::from_name("NOERROR").unwrap(), RCode::NOERROR);
        assert_eq!(RCode::from_name("FORMERR").unwrap(), RCode::FORMERR);
        assert_eq!(RCode::from_name("SERVFAIL").unwrap(), RCode::SERVFAIL);
        assert_eq!(RCode::from_name("NXDOMAIN").unwrap(), RCode::NXDOMAIN);
        assert_eq!(RCode::from_name("NOTIMP").unwrap(), RCode::NOTIMP);
        assert_eq!(RCode::from_name("REFUSED").unwrap(), RCode::REFUSED);
        assert_eq!(RCode::from_name("BADVERS").unwrap(), RCode::BADVERS);

        for (i, name) in NAMES.iter().enumerate() {
            if *name != UNKNOWN_RCODE {
                assert_eq!(RCode::from(i as u16), RCode::from_name(name).unwrap());
                assert!(RCode::from_name(&name.to_lowercase()).is_err());
            } else {
                assert!(RCode::from_name(name).is_err());
            }
        }
    }

    #[test]
    fn test_from_str() {
        assert_eq!(RCode::from_str("NOERROR").unwrap(), RCode::NOERROR);
        assert_eq!(RCode::from_str("FORMERR").unwrap(), RCode::FORMERR);
        assert_eq!(RCode::from_str("SERVFAIL").unwrap(), RCode::SERVFAIL);
        assert_eq!(RCode::from_str("NXDOMAIN").unwrap(), RCode::NXDOMAIN);
        assert_eq!(RCode::from_str("NOTIMP").unwrap(), RCode::NOTIMP);
        assert_eq!(RCode::from_str("REFUSED").unwrap(), RCode::REFUSED);
        assert_eq!(RCode::from_str("BADVERS").unwrap(), RCode::BADVERS);

        for (i, name) in NAMES.iter().enumerate() {
            if *name != UNKNOWN_RCODE {
                assert_eq!(RCode::from(i as u16), RCode::from_str(name).unwrap());
                assert!(RCode::from_str(&name.to_lowercase()).is_err());
            }
        }

        for i in 0..=u16::MAX {
            let s = format!("RCODE{}", i);
            assert_eq!(RCode::from_str(&s).unwrap(), RCode::from(i));
            assert!(RCode::from_str(&s.to_lowercase()).is_err());
        }

        assert!(RCode::from_str("RCODE65536").is_err());
    }

    #[test]
    fn test_is_defined() {
        assert!(RCode::NOERROR.is_defined());
        assert!(RCode::FORMERR.is_defined());
        assert!(RCode::SERVFAIL.is_defined());
        assert!(RCode::NXDOMAIN.is_defined());
        assert!(RCode::NOTIMP.is_defined());
        assert!(RCode::REFUSED.is_defined());
        assert!(RCode::BADVERS.is_defined());

        for v in RCode::VALUES {
            assert!(v.is_defined());
        }

        for (i, name) in NAMES.iter().enumerate() {
            assert_eq!(RCode::from(i as u16).is_defined(), *name != UNKNOWN_RCODE);
        }

        for i in 0..=u8::MAX {
            let rcode = RCode::from(i as u16);
            assert_eq!(
                rcode.is_defined(),
                RCode::VALUES.iter().any(|e| *e == rcode)
            );
        }
    }
}
