use crate::{
    constants::{DOMAIN_NAME_LABEL_MAX_LENGTH, DOMAIN_NAME_MAX_LENGTH},
    Error, Result,
};
use arrayvec::ArrayString;
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
    str::FromStr,
};

type ArrayType = ArrayString<DOMAIN_NAME_MAX_LENGTH>;

/// A domain name.
///
/// This struct models the domain name above a fixed array of [`DOMAIN_NAME_MAX_LENGTH`] bytes.
/// This is done in order to avoid dynamic memory allocation.
///
/// `DomainName` stores the name in the form `example.com.`. The trailing period denotes the root
/// zone.
///
/// Domain name max length, as defined in
/// [RFC 1035](https://tools.ietf.org/html/rfc1035#section-3.1), is 255 bytes.
/// This includes all label length bytes, and the terminating zero length byte. Hence the effective
/// max length of a domain name without the root zone is 253 bytes.
///
/// Domain name is case insensitive. Hence the implementation of `PartialEq` converts each side to
/// ASCII lowercase. Use [`DomainName::as_str`] when exact match is required.
///
/// Specifications:
///
/// - [RFC 1035 ~2.3.1](https://tools.ietf.org/html/rfc1035#section-2.3.1)
/// - [RFC 1035 ~2.3.4](https://tools.ietf.org/html/rfc1035#section-2.3.4)
/// - [RFC 1035 ~3.1](https://tools.ietf.org/html/rfc1035#section-3.1)
#[derive(Debug, Default, Clone)]
pub struct DomainName {
    arr: ArrayType,
}

impl DomainName {
    /// Creates an empty `DomainName`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    ///
    /// let dn = DomainName::new();
    ///
    /// assert_eq!(dn.len(), 0);
    /// assert!(dn.is_empty());
    /// ```
    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates the root `DomainName`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    ///
    /// let dn = DomainName::new_root();
    /// assert_eq!(dn.len(), 1);
    /// assert_eq!(dn.as_str(), ".");
    /// ```
    pub fn new_root() -> Self {
        let mut dn = Self::default();
        dn.set_root();
        dn
    }

    /// Creates a `DomainName` from a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    ///
    /// let dn = DomainName::from("example.com").unwrap();
    /// assert_eq!(dn.as_str(), "example.com.");
    ///
    /// let dn = DomainName::from("sub.example.com.").unwrap();
    /// assert_eq!(dn.as_str(), "sub.example.com.");
    /// ```
    pub fn from(s: &str) -> Result<Self> {
        Self::check_name(s)?;

        let mut dn = Self {
            // check_name verifies the length of the string,
            // so the following unwrap will not panic.
            arr: ArrayType::from_str(s).unwrap(),
        };

        let bytes = s.as_bytes();

        // check_name rejects an empty string, so it is sound to use unchecked access here
        let last_byte = unsafe { *bytes.get_unchecked(bytes.len() - 1) };

        if last_byte != b'.' {
            // check_name verifies the length of the string and ensures that
            // the root zone can be accommodated.
            // Thus the following push is sound and will not panic.
            dn.arr.push('.');
        }

        Ok(dn)
    }

    /// Returns the `DomainName` as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    ///
    /// let dn = DomainName::new();
    /// assert_eq!(dn.as_str(), "");
    ///
    /// let dn = DomainName::from("example.com").unwrap();
    /// assert_eq!(dn.as_str(), "example.com.");
    ///
    /// let dn = DomainName::from(".").unwrap();
    /// assert_eq!(dn.as_str(), ".");
    /// ```
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.arr.as_str()
    }

    /// Checks if a byte slice is a valid domain name label.
    pub fn check_label_bytes(label: &[u8]) -> Result<()> {
        if label.is_empty() {
            return Err(Error::DomainNameLabelMalformed);
        }

        let len = label.len();

        if len > DOMAIN_NAME_LABEL_MAX_LENGTH {
            return Err(Error::DomainNameLabelTooLong(len));
        }

        for b in label.iter().cloned() {
            if !(b.is_ascii_alphanumeric() || b == b'-') {
                return Err(Error::DomainNameLabelInvalidChar);
            }
        }

        // the slice is not empty (checked at the top of the function)
        // so it is sound to access it unchecked at the first and last bytes
        unsafe {
            if !label.get_unchecked(0).is_ascii_alphabetic() {
                return Err(Error::DomainNameLabelMalformed);
            }
            if !label.get_unchecked(len - 1).is_ascii_alphanumeric() {
                return Err(Error::DomainNameLabelMalformed);
            }
        }

        Ok(())
    }

    /// Checks if a string is a valid domain name label.
    ///
    /// This is a string slice equivalent of [`DomainName::check_label_bytes`].
    #[inline(always)]
    pub fn check_label(label: &str) -> Result<()> {
        Self::check_label_bytes(label.as_bytes())
    }

    /// Checks if a byte slice is a valid domain name.
    pub fn check_name_bytes(name: &[u8]) -> Result<()> {
        if name.is_empty() {
            return Err(Error::DomainNameLabelMalformed);
        }

        // root domain name
        if name == b"." {
            return Ok(());
        }

        let len = name.len();
        let mut domain_start: Option<usize> = None;

        let mut i = 0;
        for j in 0..len {
            let byte = unsafe { *name.get_unchecked(j) };
            if byte == b'.' {
                let label = unsafe { name.get_unchecked(i..j) };
                Self::check_label_bytes(label)?;
                i = j + 1;
                domain_start = Some(i);
            }
        }

        match domain_start {
            Some(ds) if len - ds > 0 => {
                let label = unsafe { name.get_unchecked(ds..len) };
                Self::check_label_bytes(label)?;
            }
            None => Self::check_label_bytes(name)?,
            _ => (),
        }

        let last_byte = unsafe { *name.get_unchecked(len - 1) };

        let effective_max_length = if last_byte == b'.' {
            DOMAIN_NAME_MAX_LENGTH - 1
        } else {
            DOMAIN_NAME_MAX_LENGTH - 2
        };

        if len > effective_max_length {
            return Err(Error::DomainNameTooLong);
        }

        Ok(())
    }

    /// Checks if a string is a valid domain name.
    ///
    /// This is a string slice equivalent of [`DomainName::check_name_bytes`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    ///
    /// assert!(DomainName::check_name("example.com").is_ok());
    /// assert!(DomainName::check_name("example-.com").is_err());
    /// assert!(DomainName::check_name("").is_err());
    ///
    /// assert!(DomainName::check_name(".").is_ok());
    /// assert!(DomainName::check_name("..").is_err());
    /// ```
    #[inline(always)]
    pub fn check_name(name: &str) -> Result<()> {
        Self::check_name_bytes(name.as_bytes())
    }

    /// Returns the length of the `DomainName`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    ///
    /// let dn = DomainName::new();
    /// assert_eq!(dn.len(), 0);
    ///
    /// let dn = DomainName::from("example.com").unwrap();
    /// assert_eq!(dn.len(), 12); // includes the root zone
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.arr.len()
    }

    /// Returns the capacity of the underlying buffer.
    ///
    /// This is a convenience method. The capacity equals [`DOMAIN_NAME_MAX_LENGTH`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::{constants::DOMAIN_NAME_MAX_LENGTH, DomainName};
    ///
    /// let dn = DomainName::from("example.com.").unwrap();
    /// assert_eq!(dn.len(), 12);
    /// assert_eq!(dn.capacity(), DOMAIN_NAME_MAX_LENGTH);
    /// ```
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.arr.capacity()
    }

    /// Checks if `DomainName` is empty.
    ///
    /// **Note**: empty domain name is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    /// use std::str::FromStr;
    ///
    /// let dn = DomainName::from_str("example.com").unwrap();
    /// assert_eq!(dn.is_empty(), false);
    ///
    /// let dn = DomainName::new();
    /// assert_eq!(dn.is_empty(), true);
    /// ```
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }

    /// Make the `DomainName` empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    /// use std::str::FromStr;
    ///
    /// let mut dn = DomainName::from_str("example.com").unwrap();
    /// assert_eq!(dn.is_empty(), false);
    /// assert_eq!(dn.len(), 12);
    /// assert_eq!(dn.as_str(), "example.com.");
    ///
    /// dn.clear();
    /// assert_eq!(dn.is_empty(), true);
    /// assert_eq!(dn.len(), 0);
    /// assert_eq!(dn.as_str(), "");
    /// ```
    #[inline(always)]
    pub fn clear(&mut self) {
        self.arr.clear();
    }

    /// Appends a label to the `DomainName`.
    ///
    /// This function is dedicated to a parser which needs to construct
    /// a domain name label by label, as they are read from the DNS on-wire representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    ///
    /// let mut dn = DomainName::new();
    ///
    /// dn.push_label_bytes(b"example").unwrap();
    /// assert_eq!(dn.as_str(), "example.");
    ///
    /// dn.push_label_bytes(b"com").unwrap();
    /// assert_eq!(dn.as_str(), "example.com.");
    /// ```
    pub fn push_label_bytes(&mut self, label: &[u8]) -> Result<()> {
        Self::check_label_bytes(label)?;

        // at this point the label is proven to be valid,
        // which means it is sound to convert it unchecked as a valid label is ASCII
        let label_as_str = unsafe { std::str::from_utf8_unchecked(label) };

        if self.arr.try_push_str(label_as_str).is_err() {
            return Err(Error::DomainNameTooLong);
        }

        if self.arr.try_push('.').is_err() {
            return Err(Error::DomainNameTooLong);
        }

        Ok(())
    }

    /// Appends a label to the `DomainName`.
    ///
    /// This is a string slice equivalent of [`DomainName::push_label_bytes`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    ///
    /// let mut dn = DomainName::new();
    ///
    /// dn.push_label("example").unwrap();
    /// assert_eq!(dn.as_str(), "example.");
    ///
    /// dn.push_label("com").unwrap();
    /// assert_eq!(dn.as_str(), "example.com.");
    /// ```
    pub fn push_label(&mut self, label: &str) -> Result<()> {
        Self::check_label(label)?;

        if self.arr.try_push_str(label).is_err() {
            return Err(Error::DomainNameTooLong);
        }

        if self.arr.try_push('.').is_err() {
            return Err(Error::DomainNameTooLong);
        }

        Ok(())
    }

    /// Sets this domain name to be the root zone domain name `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rsdns::DomainName;
    ///
    /// let mut dn = DomainName::new();
    /// assert!(dn.is_empty());
    ///
    /// dn.set_root();
    /// assert_eq!(dn.as_str(), ".");
    ///
    /// dn = DomainName::from("example.com").unwrap();
    /// assert_eq!(dn.as_str(), "example.com.");
    ///
    /// dn.set_root();
    /// assert_eq!(dn.as_str(), ".");
    /// ```
    pub fn set_root(&mut self) {
        self.arr.clear();
        self.arr.push('.');
    }
}

impl TryFrom<&str> for DomainName {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from(value)
    }
}

impl FromStr for DomainName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from(s)
    }
}

impl AsRef<str> for DomainName {
    fn as_ref(&self) -> &str {
        self.arr.as_str()
    }
}

impl PartialEq for DomainName {
    fn eq(&self, other: &Self) -> bool {
        self.arr
            .as_bytes()
            .eq_ignore_ascii_case(other.arr.as_bytes())
    }
}

impl PartialOrd for DomainName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DomainName {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.len().min(other.len()) {
            let left = unsafe { self.arr.as_bytes().get_unchecked(i) };
            let right = unsafe { other.arr.as_bytes().get_unchecked(i) };
            let ord = left.to_ascii_lowercase().cmp(&right.to_ascii_lowercase());
            if Ordering::Equal != ord {
                return ord;
            }
        }
        self.len().cmp(&other.len())
    }
}

impl PartialEq<&str> for DomainName {
    fn eq(&self, other: &&str) -> bool {
        let l_is_root = self.arr.as_bytes() == b".";
        let r_is_root = *other == ".";

        match (l_is_root, r_is_root) {
            (true, true) => return true,
            (false, false) => {}
            _ => return false,
        }

        let mut bytes = self.arr.as_bytes();
        if !bytes.is_empty() && !other.ends_with('.') {
            bytes = &bytes[..bytes.len() - 1];
        }

        bytes.eq_ignore_ascii_case(other.as_bytes())
    }
}

impl Eq for DomainName {}

impl Hash for DomainName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for b in self.arr.as_bytes() {
            state.write_u8(b.to_ascii_lowercase());
        }
    }
}

impl Display for DomainName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_new() {
        let dn = DomainName::new();

        assert!(dn.is_empty());
        assert_eq!(dn.len(), 0);
    }

    #[test]
    fn test_default() {
        let dn: DomainName = Default::default();

        assert!(dn.is_empty());
        assert_eq!(dn.len(), 0);
    }

    #[test]
    fn test_from() {
        let label_63 = "a".repeat(63);
        let label_61 = "b".repeat(60);

        let dn_253 = vec![
            label_63.as_str(),
            label_63.as_str(),
            label_63.as_str(),
            label_61.as_str(),
        ]
        .join(".");

        let dn_254 = dn_253.clone() + ".";

        let dn_255 = vec![
            label_63.as_str(),
            label_63.as_str(),
            label_63.as_str(),
            label_63.as_str(),
        ]
        .join(".");

        let success_cases = &[
            "com",
            "example.com",
            "sub.example.com",
            ".",
            "example.com.",
            "EXAMPLE.com",
            "EXAMPLE.COM",
            "EXAMPLE.COM.",
            dn_253.as_str(),
            dn_254.as_str(),
        ];

        for sc in success_cases {
            let dn = DomainName::from(sc).unwrap();
            let expected = if sc.ends_with(".") {
                sc.to_string()
            } else {
                format!("{}.", sc)
            };
            assert_eq!(dn.as_str(), &expected);
            assert_eq!(dn.len(), expected.len());
        }

        let failure_cases = &[
            "",
            "..",
            "3om",
            "co-",
            "example..com",
            "sub..example.com",
            "1xample.com",
            "example-.com",
            "-xample.com",
            "examp|e.com",
            "exa\u{203C}ple.com",
            dn_255.as_str(),
        ];

        for fc in failure_cases {
            assert!(DomainName::from(fc).is_err())
        }
    }

    #[test]
    fn test_check_label() {
        let malformed: &[&[u8]] = &[b"", b"1abel", b"-abel", b"label-"];

        for m in malformed {
            let res = DomainName::check_label_bytes(m);
            assert!(matches!(res, Err(Error::DomainNameLabelMalformed)));

            let res = DomainName::check_label(std::str::from_utf8(m).unwrap());
            assert!(matches!(res, Err(Error::DomainNameLabelMalformed)));
        }

        let invalid_char: &[&[u8]] = &[b"la.el", b"\tabel"];
        for ic in invalid_char {
            let res = DomainName::check_label_bytes(ic);
            assert!(matches!(res, Err(Error::DomainNameLabelInvalidChar)));

            let res = DomainName::check_label(std::str::from_utf8(ic).unwrap());
            assert!(matches!(res, Err(Error::DomainNameLabelInvalidChar)));
        }

        let l_64 = "a".repeat(64);
        let too_large = &[l_64.as_bytes()];
        for tl in too_large {
            let res = DomainName::check_label_bytes(tl);
            assert!(matches!(res, Err(Error::DomainNameLabelTooLong(l)) if l == tl.len()));

            let res = DomainName::check_label(std::str::from_utf8(tl).unwrap());
            assert!(matches!(res, Err(Error::DomainNameLabelTooLong(l)) if l == tl.len()));
        }

        let l_63 = "a".repeat(63);
        let good: &[&[u8]] = &[b"label", b"labe1", l_63.as_bytes()];
        for g in good {
            assert!(DomainName::check_label_bytes(g).is_ok());
            assert!(DomainName::check_label(std::str::from_utf8(g).unwrap()).is_ok());
        }
    }

    #[test]
    fn test_check_name() {
        let good: &[&[u8]] = &[
            b".",
            b"com",
            b"example.com",
            b"exampl0.com.",
            b"exam-3le.com",
            b"su--b.exAmp1e.com",
        ];
        for g in good {
            assert!(DomainName::check_name_bytes(g).is_ok());
            assert!(DomainName::check_name(std::str::from_utf8(g).unwrap()).is_ok());
        }

        let malformed: &[&[u8]] = &[
            b"",
            b"..",
            b"3om",
            b"co-",
            b"example.com..",
            b"example..com",
            b"sub..example.com",
            b"1xample.com",
            b"example-.com",
            b"-xample.com",
        ];

        for m in malformed {
            let res = DomainName::check_name_bytes(m);
            assert!(matches!(res, Err(Error::DomainNameLabelMalformed)));

            let res = DomainName::check_name(std::str::from_utf8(m).unwrap());
            assert!(matches!(res, Err(Error::DomainNameLabelMalformed)));
        }

        let invalid_char: &[&[u8]] = &[b"examp|e.com."];

        for ic in invalid_char {
            let res = DomainName::check_name_bytes(ic);
            assert!(matches!(res, Err(Error::DomainNameLabelInvalidChar)));

            let res = DomainName::check_name(std::str::from_utf8(ic).unwrap());
            assert!(matches!(res, Err(Error::DomainNameLabelInvalidChar)));
        }

        let l_63 = "a".repeat(63);
        let l_61 = "b".repeat(61);
        let dn_253 = vec![l_63.clone(), l_63.clone(), l_63.clone()].join(".") + "." + l_61.as_str();
        let dn_254 = dn_253.clone() + "b";

        assert!(DomainName::check_name_bytes(dn_253.as_str().as_bytes()).is_ok());
        assert!(DomainName::check_name(dn_253.as_str()).is_ok());
        assert!(DomainName::check_name_bytes((dn_253.clone() + ".").as_str().as_bytes()).is_ok());
        assert!(DomainName::check_name((dn_253.clone() + ".").as_str()).is_ok());

        let too_long = &[dn_254.as_str()];
        for tl in too_long {
            let res = DomainName::check_name(tl);
            assert!(matches!(res, Err(Error::DomainNameTooLong)));

            let res = DomainName::check_name_bytes(tl.as_bytes());
            assert!(matches!(res, Err(Error::DomainNameTooLong)));
        }
    }

    #[test]
    fn test_len() {
        let mut dn = DomainName::new();
        assert_eq!(dn.len(), 0);

        dn.push_label("example").unwrap();
        assert_eq!(dn.len(), 8);

        dn.push_label("com").unwrap();
        assert_eq!(dn.len(), 12);
    }

    #[test]
    fn test_push_label_too_long() {
        let l_63 = "a".repeat(63);
        let l_62 = "b".repeat(62);

        let mut dn = DomainName::new();

        dn.push_label(&l_63).unwrap();
        assert_eq!(dn.len(), 64);

        dn.push_label(&l_63).unwrap();
        assert_eq!(dn.len(), 128);

        dn.push_label(&l_63).unwrap();
        assert_eq!(dn.len(), 192);

        // test total size > 255
        {
            let mut dn = dn.clone();
            dn.push_label("small").unwrap();

            let res = dn.push_label(&l_63);
            assert!(matches!(res, Err(Error::DomainNameTooLong)));
        }

        // test total size == 255
        let res = dn.clone().push_label(&l_63);
        assert!(matches!(res, Err(Error::DomainNameTooLong)));

        dn.push_label(&l_62).unwrap();
        assert_eq!(dn.len(), 255);
    }

    #[test]
    fn test_eq() {
        let dn1 = DomainName::from("example.com").unwrap();
        let dn2 = DomainName::from("EXAMPLE.COM").unwrap();
        let dn3 = DomainName::from("eXaMpLe.cOm").unwrap();

        assert_eq!(dn1, dn2);
        assert_eq!(dn1, dn3);
        assert_eq!(dn2, dn3);
    }

    #[test]
    fn test_neq() {
        let dn1 = DomainName::from("example.com").unwrap();
        let dn2 = DomainName::from("sub.example.com").unwrap();
        let dn3 = DomainName::from("Sub.examp1e.com").unwrap();

        assert_ne!(dn1, dn2);
        assert_ne!(dn1, dn3);
        assert_ne!(dn2, dn3);
    }

    #[test]
    fn test_eq_str() {
        let dn1 = DomainName::from("example.com").unwrap();
        let dn2 = DomainName::from("EXAMPLE.COM").unwrap();

        assert_eq!(dn1, "EXAMPLE.COM.");
        assert_eq!(dn1, "EXAMPLE.COM");

        assert_eq!(dn1, "eXaMpLe.cOm.");
        assert_eq!(dn2, "eXaMpLe.cOm");

        assert_eq!(dn2, "eXaMpLe.cOm");
        assert_eq!(dn2, "eXaMpLe.cOm.");

        assert_eq!(
            DomainName::from("sub.example.com").unwrap(),
            "sub.example.com."
        );
        assert_eq!(
            DomainName::from("sub.example.com.").unwrap(),
            "sub.example.com"
        );

        assert_eq!(DomainName::new(), "");
        assert_eq!(DomainName::new_root(), ".");
    }

    #[test]
    fn test_neq_str() {
        let dn1 = DomainName::from("example.com").unwrap();
        let dn2 = DomainName::from("sub.example.com").unwrap();

        assert_ne!(dn1, "sub.example.com");
        assert_ne!(dn1, "sub.example.com.");

        assert_ne!(dn1, "Sub.examp1e.com");
        assert_ne!(dn1, "Sub.examp1e.com.");

        assert_ne!(dn2, "Sub.examp1e.com");
        assert_ne!(dn2, "Sub.examp1e.com.");

        assert_ne!(DomainName::new(), ".");
        assert_ne!(DomainName::new_root(), "");
    }

    #[test]
    fn test_hash() {
        let dn = DomainName::from("example.com").unwrap();

        let mut s = HashSet::new();
        s.insert(dn);

        assert!(s.contains(&DomainName::from("example.com.").unwrap()));
        assert!(s.contains(&DomainName::from("eXaMpLe.COM").unwrap()));
        assert!(s.contains(&DomainName::from("EXAMPLE.COM").unwrap()));

        assert!(!s.contains(&DomainName::from("suB.Example.com.").unwrap()));
    }

    #[test]
    fn test_ord() {
        let dn1 = DomainName::from("example.com").unwrap();
        let dn2 = DomainName::from("ExaMplE.com").unwrap();
        let dn3 = DomainName::from("Sub.example.com").unwrap();

        assert_eq!(Ordering::Equal, dn1.cmp(&dn2));
        assert_eq!(Ordering::Less, dn1.cmp(&dn3));
        assert_eq!(Ordering::Greater, dn3.cmp(&dn1));
        assert_eq!(
            Ordering::Equal,
            DomainName::new_root().cmp(&DomainName::new_root())
        );
        assert_eq!(Ordering::Equal, DomainName::new().cmp(&DomainName::new()));
    }

    #[test]
    fn test_partial_ord() {
        let dn1 = DomainName::from("example.com").unwrap();
        let dn2 = DomainName::from("ExaMplE.com").unwrap();
        let dn3 = DomainName::from("Sub.example.com").unwrap();

        assert_eq!(Some(Ordering::Equal), dn1.partial_cmp(&dn2));
        assert_eq!(Some(Ordering::Less), dn1.partial_cmp(&dn3));
        assert_eq!(Some(Ordering::Greater), dn3.partial_cmp(&dn1));
        assert_eq!(
            Some(Ordering::Equal),
            DomainName::new_root().partial_cmp(&DomainName::new_root())
        );
        assert_eq!(
            Some(Ordering::Equal),
            DomainName::new().partial_cmp(&DomainName::new())
        );
    }
}
