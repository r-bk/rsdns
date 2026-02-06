use crate::{
    Result,
    bytes::{Cursor, Reader},
    errors::{TypeFromStrError, UnknownTypeName},
};
use core::{
    cmp::Ordering,
    fmt::{self, Display, Formatter, Write},
    str::FromStr,
};

const UNKNOWN_TYPE: &str = "__UNKNOWN_TYPE__";
const RFC3597_PFX: &str = "TYPE";

#[rustfmt::skip]
static NAMES: [&str; 256] = [
    /*  0 */ "", "A", "NS", "MD", "MF", "CNAME", "SOA", "MB", "MG", "MR", "NULL", "WKS", "PTR", "HINFO", "MINFO", "MX",
    /*  1 */ "TXT", "", "", "", "", "", "", "", "", "", "", "", "AAAA", "", "", "",
    /*  2 */ "", "", "", "", "", "", "", "", "", "OPT", "", "", "", "", "", "",
    /*  3 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /*  4 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /*  5 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /*  6 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /*  7 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /*  8 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /*  9 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /* 10 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /* 11 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /* 12 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /* 13 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /* 14 */ "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
    /* 15 */ "", "", "", "", "", "", "", "", "", "", "", "", "AXFR", "MAILB", "MAILA", "ANY",
];

#[rustfmt::skip]
static KNOWN: [u8; 256] = [
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
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
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1,
];

/// DNS record type.
///
/// This struct represents a `TYPE`[^rfc] value.
///
/// [`Type`] is a newtype encapsulating a `u16`. It has associated constants
/// that define the values currently supported by `rsdns`. Additionally, it
/// supports values currently not having a defined named constant, following
/// [RFC 3597].
///
/// [`Type`] follows [RFC 3597 section 5] to display unknown values.
///
/// # Examples
///
/// ```rust
/// # use rsdns::records::Type;
/// # use std::{error::Error, str::FromStr};
/// # fn foo() -> Result<(), Box<dyn Error>> {
/// // Type can be created from a u16
/// assert_eq!(Type::from(1), Type::A);
/// assert_eq!(Type::from(255), Type::ANY);
///
/// // Type can be created from a string
/// assert_eq!(Type::from_name("AAAA")?, Type::AAAA);
/// assert_eq!(Type::from_str("CNAME")?, Type::CNAME);
/// assert_eq!(Type::from_str("TYPE41")?, Type::OPT);
/// assert_eq!(Type::from_str("TYPE300")?, Type::from(300));
///
/// // Type's numerical value can be obtained as u16
/// assert_eq!(Type::A.value(), 1);
/// assert_eq!(Type::AAAA.value(), 28);
///
/// // Type name is accessible in constant time
/// assert_eq!(Type::AAAA.name(), "AAAA");
/// assert_eq!(Type::from(1000).name(), "__UNKNOWN_TYPE__");
///
/// // Display implementation follows RFC3597
/// assert_eq!(format!("{}", Type::TXT), "TXT");
/// assert_eq!(format!("{}", Type::from(2900)), "TYPE2900");
/// assert_eq!(Type::from(1000).to_string(), "TYPE1000");
///
/// // Type can be compared to u16
/// assert_eq!(Type::from(28), 28);
/// assert!(Type::AAAA < 41);
/// # Ok(())
/// # }
/// # foo().unwrap();
/// ```
///
/// [^rfc]: [RFC 1035 section 3.2.2](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.2)
///
/// [RFC 3597]: https://www.rfc-editor.org/rfc/rfc3597.html
/// [RFC 3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Type(u16);

impl Type {
    /// a host address (IPv4)
    pub const A: Type = Type::new(1);

    /// an authoritative name server
    pub const NS: Type = Type::new(2);

    /// a mail destination (obsolete - use [`Type::MX`])
    pub const MD: Type = Type::new(3);

    /// a mail forwarder (obsolete - use [`Type::MX`])
    pub const MF: Type = Type::new(4);

    /// the canonical name of an alias
    pub const CNAME: Type = Type::new(5);

    /// marks the start of a zone authority
    pub const SOA: Type = Type::new(6);

    /// a mailbox domain name
    pub const MB: Type = Type::new(7);

    /// a mail group member
    pub const MG: Type = Type::new(8);

    /// a mail rename domain name
    pub const MR: Type = Type::new(9);

    /// a NULL RR
    pub const NULL: Type = Type::new(10);

    /// a well known service description
    pub const WKS: Type = Type::new(11);

    /// a domain name pointer
    pub const PTR: Type = Type::new(12);

    /// host information
    pub const HINFO: Type = Type::new(13);

    /// mailbox or mail list information
    pub const MINFO: Type = Type::new(14);

    /// mail exchange
    pub const MX: Type = Type::new(15);

    /// text strings
    pub const TXT: Type = Type::new(16);

    /// a host address (IPv6)
    /// [RFC 3596 section 2.1](https://www.rfc-editor.org/rfc/rfc3596.html#section-2.1)
    pub const AAAA: Type = Type::new(28);

    /// EDNS(0) OPT pseudo-record [RFC 6891](https://www.rfc-editor.org/rfc/rfc6891.html#section-6)
    pub const OPT: Type = Type::new(41);

    /// a request for a transfer of an entire zone
    pub const AXFR: Type = Type::new(252);

    /// a request for mailbox-related records (MB, MG or MR)
    pub const MAILB: Type = Type::new(253);

    /// a request for mail agent RRs (Obsolete - see [`Type::MX`])
    pub const MAILA: Type = Type::new(254);

    /// a request for all records
    pub const ANY: Type = Type::new(255);

    #[cfg(test)]
    #[allow(missing_docs)]
    pub const VALUES: [Type; 22] = [
        Self::A,
        Self::NS,
        Self::MD,
        Self::MF,
        Self::CNAME,
        Self::SOA,
        Self::MB,
        Self::MG,
        Self::MR,
        Self::NULL,
        Self::WKS,
        Self::PTR,
        Self::HINFO,
        Self::MINFO,
        Self::MX,
        Self::TXT,
        Self::AAAA,
        Self::OPT,
        Self::AXFR,
        Self::MAILB,
        Self::MAILA,
        Self::ANY,
    ];

    #[inline]
    const fn new(v: u16) -> Self {
        Self(v)
    }

    /// Returns the name of a Type in constant time.
    ///
    /// If the Type value doesn't have a defined named constant, the string
    /// `"__UNKNOWN_TYPE__"` is returned.
    ///
    /// To convert a Type to string following [RFC3597 section 5] see
    /// [`Type::fmt`].
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Type;
    /// assert_eq!(Type::CNAME.name(), "CNAME");
    /// assert_eq!(Type::from(u16::MAX).name(), "__UNKNOWN_TYPE__");
    /// ```
    ///
    /// [RFC3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    #[inline]
    pub fn name(self) -> &'static str {
        let val = self.value() as usize;
        let name_ = if val < NAMES.len() { NAMES[val] } else { "" };
        match name_ {
            "" => UNKNOWN_TYPE,
            _ => name_,
        }
    }

    /// Returns the Type numerical value as `u16`.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Type;
    /// assert_eq!(Type::AAAA.value(), 28);
    /// assert_eq!(Type::from(333).value(), 333);
    /// ```
    #[inline]
    pub const fn value(self) -> u16 {
        self.0
    }

    /// Checks if this is a data-type value.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Type;
    /// assert_eq!(Type::A.is_data_type(), true);
    /// assert_eq!(Type::ANY.is_data_type(), false);
    /// assert_eq!(Type::from(u16::MAX).is_data_type(), false);
    /// ```
    #[inline]
    pub fn is_data_type(self) -> bool {
        (0x0001 <= self.0 && self.0 <= 0x007F) || (0x0100 <= self.0 && self.0 <= 0xEFFF)
    }

    /// Checks if this is a question-type or a meta-type value.
    ///
    /// [RFC 6895 section 3.1](https://www.rfc-editor.org/rfc/rfc6895.html#section-3.1)
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Type;
    /// assert_eq!(Type::ANY.is_meta_type(), true);
    /// assert_eq!(Type::A.is_meta_type(), false);
    /// assert_eq!(Type::from(u16::MAX).is_meta_type(), false);
    /// ```
    #[inline]
    pub fn is_meta_type(self) -> bool {
        0x0080 <= self.0 && self.0 <= 0x00FF
    }

    /// Creates a Type from a type name.
    ///
    /// `name` is expected to be an all-capital name of a type,
    /// as returned from [`Type::name`]. Names of types with no defined named
    /// constant are not recognized. The comparison is case-sensitive.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{records::Type, errors::UnknownTypeName};
    /// # fn foo() -> Result<(), UnknownTypeName> {
    /// assert_eq!(Type::from_name("CNAME")?, Type::CNAME);
    /// assert!(Type::from_name("UNKNOWN").is_err());
    /// assert!(Type::from_name("Cname").is_err());
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - [`UnknownTypeName`] - the type name is not recognized
    pub fn from_name(name: &str) -> core::result::Result<Self, UnknownTypeName> {
        match name.len() {
            1 => match name {
                "A" => Ok(Type::A),
                _ => Err(UnknownTypeName),
            },
            2 => match name {
                "NS" => Ok(Type::NS),
                "MX" => Ok(Type::MX),
                "MD" => Ok(Type::MD),
                "MF" => Ok(Type::MF),
                "MB" => Ok(Type::MB),
                "MG" => Ok(Type::MG),
                "MR" => Ok(Type::MR),
                _ => Err(UnknownTypeName),
            },
            3 => match name {
                "SOA" => Ok(Type::SOA),
                "TXT" => Ok(Type::TXT),
                "OPT" => Ok(Type::OPT),
                "PTR" => Ok(Type::PTR),
                "ANY" => Ok(Type::ANY),
                "WKS" => Ok(Type::WKS),
                _ => Err(UnknownTypeName),
            },
            4 => match name {
                "AAAA" => Ok(Type::AAAA),
                "NULL" => Ok(Type::NULL),
                "AXFR" => Ok(Type::AXFR),
                _ => Err(UnknownTypeName),
            },
            5 => match name {
                "CNAME" => Ok(Type::CNAME),
                "HINFO" => Ok(Type::HINFO),
                "MINFO" => Ok(Type::MINFO),
                "MAILB" => Ok(Type::MAILB),
                "MAILA" => Ok(Type::MAILA),
                _ => Err(UnknownTypeName),
            },
            _ => Err(UnknownTypeName),
        }
    }

    /// Checks if the Type value equals one of the defined named constants.
    ///
    /// This is implemented as an `O(1)` operation.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Type;
    /// assert!(Type::AAAA.is_defined());
    /// assert!(!Type::from(1000).is_defined());
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

impl From<u16> for Type {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Display for Type {
    /// Formats a Type as specified in [RFC3597 section 5].
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::records::Type;
    /// assert_eq!(format!("{}", Type::AAAA), "AAAA");
    /// assert_eq!(format!("{}", Type::from(2000)), "TYPE2000");
    /// assert_eq!(Type::from(1111).to_string(), "TYPE1111");
    /// ```
    ///
    /// [RFC3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = self.name();
        match name {
            UNKNOWN_TYPE => {
                let mut buf = arrayvec::ArrayString::<32>::new();
                write!(&mut buf, "{}{}", RFC3597_PFX, self.0)?;
                f.pad(buf.as_str())
            }
            _ => f.pad(name),
        }
    }
}

impl PartialEq<u16> for Type {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u16> for Type {
    #[inline]
    fn partial_cmp(&self, other: &u16) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Reader<Type> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> Result<Type> {
        Ok(Type::from(self.u16_be()?))
    }
}

impl FromStr for Type {
    type Err = TypeFromStrError;

    /// Creates a Type from a string.
    ///
    /// The string `s` is expected to be a Type name or a display value
    /// as returned from [`Type::fmt`] for unknown types.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::{records::Type, errors::TypeFromStrError};
    /// # use core::str::FromStr;
    /// # fn foo() -> Result<(), TypeFromStrError> {
    /// assert_eq!(Type::from_str("AAAA")?, Type::AAAA);
    /// assert_eq!(Type::from_str("TYPE41")?, Type::OPT);
    /// assert_eq!(Type::from_str("TYPE300")?, Type::from(300));
    ///
    /// assert!(Type::from_str("Aaaa").is_err());
    /// assert!(Type::from_str("Type41").is_err());
    /// assert!(Type::from_str("TYPE65536").is_err());  // exceeds u16::MAX
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    #[inline]
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        if let Ok(t) = Self::from_name(s) {
            return Ok(t);
        }
        if let Some(sfx) = s.strip_prefix(RFC3597_PFX) {
            match sfx.parse::<u16>() {
                Ok(v) => Ok(Type::from(v)),
                _ => Err(TypeFromStrError),
            }
        } else {
            Err(TypeFromStrError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(Type::A.name(), "A");
        assert_eq!(Type::NS.name(), "NS");
        assert_eq!(Type::MD.name(), "MD");
        assert_eq!(Type::MF.name(), "MF");
        assert_eq!(Type::CNAME.name(), "CNAME");
        assert_eq!(Type::SOA.name(), "SOA");
        assert_eq!(Type::MB.name(), "MB");
        assert_eq!(Type::MG.name(), "MG");
        assert_eq!(Type::MR.name(), "MR");
        assert_eq!(Type::NULL.name(), "NULL");
        assert_eq!(Type::WKS.name(), "WKS");
        assert_eq!(Type::PTR.name(), "PTR");
        assert_eq!(Type::HINFO.name(), "HINFO");
        assert_eq!(Type::MINFO.name(), "MINFO");
        assert_eq!(Type::MX.name(), "MX");
        assert_eq!(Type::TXT.name(), "TXT");
        assert_eq!(Type::AAAA.name(), "AAAA");
        assert_eq!(Type::OPT.name(), "OPT");
        assert_eq!(Type::AXFR.name(), "AXFR");
        assert_eq!(Type::MAILB.name(), "MAILB");
        assert_eq!(Type::MAILA.name(), "MAILA");
        assert_eq!(Type::ANY.name(), "ANY");

        for (i, name) in NAMES.iter().enumerate() {
            let type_ = Type::from(i as u16);
            match type_ {
                Type::A => assert_eq!(Type::A.name(), *name),
                Type::NS => assert_eq!(Type::NS.name(), *name),
                Type::MD => assert_eq!(Type::MD.name(), *name),
                Type::MF => assert_eq!(Type::MF.name(), *name),
                Type::CNAME => assert_eq!(Type::CNAME.name(), *name),
                Type::SOA => assert_eq!(Type::SOA.name(), *name),
                Type::MB => assert_eq!(Type::MB.name(), *name),
                Type::MG => assert_eq!(Type::MG.name(), *name),
                Type::MR => assert_eq!(Type::MR.name(), *name),
                Type::NULL => assert_eq!(Type::NULL.name(), *name),
                Type::WKS => assert_eq!(Type::WKS.name(), *name),
                Type::PTR => assert_eq!(Type::PTR.name(), *name),
                Type::HINFO => assert_eq!(Type::HINFO.name(), *name),
                Type::MINFO => assert_eq!(Type::MINFO.name(), *name),
                Type::MX => assert_eq!(Type::MX.name(), *name),
                Type::TXT => assert_eq!(Type::TXT.name(), *name),
                Type::AAAA => assert_eq!(Type::AAAA.name(), *name),
                Type::OPT => assert_eq!(Type::OPT.name(), *name),
                Type::AXFR => assert_eq!(Type::AXFR.name(), *name),
                Type::MAILB => assert_eq!(Type::MAILB.name(), *name),
                Type::MAILA => assert_eq!(Type::MAILA.name(), *name),
                Type::ANY => assert_eq!(Type::ANY.name(), *name),
                _ => assert_eq!("", *name),
            }
        }
    }

    #[test]
    fn test_from_name() {
        assert_eq!(Type::from_name("A").unwrap(), Type::A);
        assert_eq!(Type::from_name("NS").unwrap(), Type::NS);
        assert_eq!(Type::from_name("MD").unwrap(), Type::MD);
        assert_eq!(Type::from_name("MF").unwrap(), Type::MF);
        assert_eq!(Type::from_name("CNAME").unwrap(), Type::CNAME);
        assert_eq!(Type::from_name("SOA").unwrap(), Type::SOA);
        assert_eq!(Type::from_name("MB").unwrap(), Type::MB);
        assert_eq!(Type::from_name("MG").unwrap(), Type::MG);
        assert_eq!(Type::from_name("MR").unwrap(), Type::MR);
        assert_eq!(Type::from_name("NULL").unwrap(), Type::NULL);
        assert_eq!(Type::from_name("WKS").unwrap(), Type::WKS);
        assert_eq!(Type::from_name("PTR").unwrap(), Type::PTR);
        assert_eq!(Type::from_name("HINFO").unwrap(), Type::HINFO);
        assert_eq!(Type::from_name("MINFO").unwrap(), Type::MINFO);
        assert_eq!(Type::from_name("MX").unwrap(), Type::MX);
        assert_eq!(Type::from_name("TXT").unwrap(), Type::TXT);
        assert_eq!(Type::from_name("AAAA").unwrap(), Type::AAAA);
        assert_eq!(Type::from_name("OPT").unwrap(), Type::OPT);
        assert_eq!(Type::from_name("AXFR").unwrap(), Type::AXFR);
        assert_eq!(Type::from_name("MAILB").unwrap(), Type::MAILB);
        assert_eq!(Type::from_name("MAILA").unwrap(), Type::MAILA);
        assert_eq!(Type::from_name("ANY").unwrap(), Type::ANY);

        for (i, name) in NAMES.iter().enumerate() {
            if !name.is_empty() {
                assert_eq!(Type::from_name(name).unwrap(), Type::from(i as u16));
                assert!(Type::from_name(&name.to_lowercase()).is_err());
            }
        }
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Type::from_str("A").unwrap(), Type::A);
        assert_eq!(Type::from_str("NS").unwrap(), Type::NS);
        assert_eq!(Type::from_str("MD").unwrap(), Type::MD);
        assert_eq!(Type::from_str("MF").unwrap(), Type::MF);
        assert_eq!(Type::from_str("CNAME").unwrap(), Type::CNAME);
        assert_eq!(Type::from_str("SOA").unwrap(), Type::SOA);
        assert_eq!(Type::from_str("MB").unwrap(), Type::MB);
        assert_eq!(Type::from_str("MG").unwrap(), Type::MG);
        assert_eq!(Type::from_str("MR").unwrap(), Type::MR);
        assert_eq!(Type::from_str("NULL").unwrap(), Type::NULL);
        assert_eq!(Type::from_str("WKS").unwrap(), Type::WKS);
        assert_eq!(Type::from_str("PTR").unwrap(), Type::PTR);
        assert_eq!(Type::from_str("HINFO").unwrap(), Type::HINFO);
        assert_eq!(Type::from_str("MINFO").unwrap(), Type::MINFO);
        assert_eq!(Type::from_str("MX").unwrap(), Type::MX);
        assert_eq!(Type::from_str("TXT").unwrap(), Type::TXT);
        assert_eq!(Type::from_str("AAAA").unwrap(), Type::AAAA);
        assert_eq!(Type::from_str("OPT").unwrap(), Type::OPT);
        assert_eq!(Type::from_str("AXFR").unwrap(), Type::AXFR);
        assert_eq!(Type::from_str("MAILB").unwrap(), Type::MAILB);
        assert_eq!(Type::from_str("MAILA").unwrap(), Type::MAILA);
        assert_eq!(Type::from_str("ANY").unwrap(), Type::ANY);

        for (i, name) in NAMES.iter().enumerate() {
            if !name.is_empty() {
                assert_eq!(Type::from_str(name).unwrap(), Type::from(i as u16));
                assert!(Type::from_str(&name.to_lowercase()).is_err());
            }
        }

        for i in 0..=u16::MAX {
            let s = format!("TYPE{i}");
            assert_eq!(Type::from_str(&s).unwrap(), Type::from(i));
            assert!(Type::from_str(&s.to_lowercase()).is_err());
        }
    }

    #[test]
    fn test_is_defined() {
        assert!(Type::A.is_defined());
        assert!(Type::NS.is_defined());
        assert!(Type::MD.is_defined());
        assert!(Type::MF.is_defined());
        assert!(Type::CNAME.is_defined());
        assert!(Type::SOA.is_defined());
        assert!(Type::MB.is_defined());
        assert!(Type::MG.is_defined());
        assert!(Type::MR.is_defined());
        assert!(Type::NULL.is_defined());
        assert!(Type::WKS.is_defined());
        assert!(Type::PTR.is_defined());
        assert!(Type::HINFO.is_defined());
        assert!(Type::MINFO.is_defined());
        assert!(Type::MX.is_defined());
        assert!(Type::TXT.is_defined());
        assert!(Type::AAAA.is_defined());
        assert!(Type::OPT.is_defined());
        assert!(Type::AXFR.is_defined());
        assert!(Type::MAILB.is_defined());
        assert!(Type::MAILA.is_defined());
        assert!(Type::ANY.is_defined());

        for (i, name) in NAMES.iter().enumerate() {
            assert_eq!(Type::from(i as u16).is_defined(), !name.is_empty());
        }

        for i in 0..=256 {
            assert_eq!(
                Type::from(i).is_defined(),
                Type::VALUES.iter().any(|v| *v == i),
            );
        }
    }
}
