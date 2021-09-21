use crate::{
    bytes::{Cursor, Reader},
    constants::DOMAIN_NAME_MAX_LENGTH,
    names::Name,
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

/// A domain name backed by byte array.
///
/// This struct implements the domain name using an array of bytes with capacity large enough to
/// accommodate the longest domain name allowed by the DNS protocol.
///
/// It is used in cases when dynamic memory allocation is undesirable. In particular, [rsdns]
/// uses it in resource record header. As a consequence parsing of resource records with no
/// variable size fields (e.g. [A], [AAAA]) involves no memory allocation at all.
///
/// [InlineName] stores the name in the canonical form `example.com.`.
/// The trailing period denotes the root DNS zone.
///
/// Domain name max length, as defined in [RFC 1035], is 255 bytes.
/// This includes all label length bytes, and the terminating zero length byte. Hence the effective
/// max length of a domain name without the root zone is 253 bytes.
///
/// Domain name is case insensitive. Hence, when compared, both sides are converted to
/// ASCII lowercase. Use [`InlineName::as_str`] when exact match is required.
///
/// Specifications:
///
/// - [RFC 1035 section 2.3.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-2.3.1)
/// - [RFC 1035 section 2.3.4](https://www.rfc-editor.org/rfc/rfc1035.html#section-2.3.4)
/// - [RFC 1035 section 3.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.1)
/// - [RFC 1101 section 3.1](https://www.rfc-editor.org/rfc/rfc1101.html#section-3.1)
///
/// [RFC 1035]: https://www.rfc-editor.org/rfc/rfc1035.html#section-3.1
/// [rsdns]: crate
/// [A]: crate::records::data::A
/// [AAAA]: crate::records::data::Aaaa
#[derive(Debug, Default, Clone)]
pub struct InlineName {
    arr: ArrayType,
}

impl InlineName {
    /// Creates an empty domain name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::names::InlineName;
    /// #
    /// let dn = InlineName::new();
    /// assert_eq!(dn.len(), 0);
    /// assert!(dn.is_empty());
    /// ```
    #[inline(always)]
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates the root domain name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::names::InlineName;
    /// #
    /// let dn = InlineName::root();
    /// assert_eq!(dn.len(), 1);
    /// assert_eq!(dn.as_str(), ".");
    /// ```
    pub fn root() -> Self {
        let mut dn = Self::default();
        dn.set_root();
        dn
    }

    fn from(s: &str) -> Result<Self> {
        super::check_name(s)?;

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

    /// Returns the domain name as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::names::InlineName;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// let dn = InlineName::new();
    /// assert_eq!(dn.as_str(), "");
    ///
    /// let dn = InlineName::from_str("example.com")?;
    /// assert_eq!(dn.as_str(), "example.com.");
    ///
    /// let dn = InlineName::from_str(".")?;
    /// assert_eq!(dn.as_str(), ".");
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.arr.as_str()
    }

    /// Returns the length of the domain name in bytes.
    ///
    /// Valid domain names are comprised of ASCII characters only.
    /// Thus this value equals the number of characters in the domain name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::names::InlineName;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let dn = InlineName::new();
    /// assert_eq!(dn.len(), 0);
    ///
    /// let dn = InlineName::from_str("example.com")?;
    /// assert_eq!(dn.len(), 12); // includes the root zone
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.arr.len()
    }

    /// Checks if domain name is empty.
    ///
    /// **Note**: empty domain name is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::names::InlineName;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let dn = InlineName::from_str("example.com")?;
    /// assert_eq!(dn.is_empty(), false);
    ///
    /// let dn = InlineName::new();
    /// assert_eq!(dn.is_empty(), true);
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }

    /// Make the domain name empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::names::InlineName;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let mut dn = InlineName::from_str("example.com")?;
    /// assert_eq!(dn.is_empty(), false);
    /// assert_eq!(dn.len(), 12);
    /// assert_eq!(dn.as_str(), "example.com.");
    ///
    /// dn.clear();
    /// assert_eq!(dn.is_empty(), true);
    /// assert_eq!(dn.len(), 0);
    /// assert_eq!(dn.as_str(), "");
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    #[inline(always)]
    pub fn clear(&mut self) {
        self.arr.clear();
    }

    pub(crate) fn append_label_bytes(&mut self, label: &[u8]) -> Result<()> {
        super::check_label_bytes(label)?;

        // at this point the label is proven to be valid,
        // which means it is sound to convert it unchecked as a valid label is ASCII
        let label_as_str = unsafe { std::str::from_utf8_unchecked(label) };

        if self.arr.try_push_str(label_as_str).is_err() {
            return Err(Error::DomainNameTooLong(
                self.arr.len() + label_as_str.len() + 1,
            ));
        }

        if self.arr.try_push('.').is_err() {
            return Err(Error::DomainNameTooLong(self.arr.len() + 1));
        }

        Ok(())
    }

    pub(crate) fn append_label(&mut self, label: &str) -> Result<()> {
        super::check_label(label)?;

        if self.arr.try_push_str(label).is_err() {
            return Err(Error::DomainNameTooLong(self.arr.len() + label.len() + 1));
        }

        if self.arr.try_push('.').is_err() {
            return Err(Error::DomainNameTooLong(self.arr.len() + 1));
        }

        Ok(())
    }

    /// Sets the domain name to denote the root DNS zone `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::names::InlineName;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let mut dn = InlineName::new();
    /// assert!(dn.is_empty());
    ///
    /// dn.set_root();
    /// assert_eq!(dn.as_str(), ".");
    ///
    /// dn = InlineName::from_str("example.com")?;
    /// assert_eq!(dn.as_str(), "example.com.");
    ///
    /// dn.set_root();
    /// assert_eq!(dn.as_str(), ".");
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    pub fn set_root(&mut self) {
        self.arr.clear();
        self.arr.push('.');
    }
}

impl TryFrom<&str> for InlineName {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from(value)
    }
}

impl FromStr for InlineName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from(s)
    }
}

impl AsRef<str> for InlineName {
    fn as_ref(&self) -> &str {
        self.arr.as_str()
    }
}

impl PartialEq for InlineName {
    fn eq(&self, other: &Self) -> bool {
        self.arr
            .as_bytes()
            .eq_ignore_ascii_case(other.arr.as_bytes())
    }
}

impl PartialOrd for InlineName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InlineName {
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

impl PartialEq<&str> for InlineName {
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

impl PartialEq<Name> for InlineName {
    #[inline]
    fn eq(&self, other: &Name) -> bool {
        self.arr
            .as_bytes()
            .eq_ignore_ascii_case(other.as_str().as_bytes())
    }
}

impl PartialEq<&Name> for InlineName {
    #[inline]
    fn eq(&self, other: &&Name) -> bool {
        self.arr
            .as_bytes()
            .eq_ignore_ascii_case(other.as_str().as_bytes())
    }
}

impl Eq for InlineName {}

impl Hash for InlineName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for b in self.arr.as_bytes() {
            state.write_u8(b.to_ascii_lowercase());
        }
    }
}

impl Display for InlineName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.as_str())
    }
}

impl From<Name> for InlineName {
    fn from(name: Name) -> Self {
        Self {
            // Name is a valid domain name, so this unwrap is not expected to panic
            arr: ArrayType::from(name.as_str()).unwrap(),
        }
    }
}

impl super::private::DNameBase for InlineName {
    #[inline(always)]
    fn as_str(&self) -> &str {
        self.as_str()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.clear()
    }

    #[inline(always)]
    fn append_label_bytes(&mut self, label: &[u8]) -> Result<()> {
        self.append_label_bytes(label)
    }

    #[inline(always)]
    fn append_label(&mut self, label: &str) -> Result<()> {
        self.append_label(label)
    }

    #[inline(always)]
    fn set_root(&mut self) {
        self.set_root()
    }

    #[inline(always)]
    fn from_cursor(c: &mut Cursor<'_>) -> Result<Self> {
        c.read()
    }
}

impl super::DName for InlineName {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_new() {
        let dn = InlineName::new();

        assert!(dn.is_empty());
        assert_eq!(dn.len(), 0);
    }

    #[test]
    fn test_default() {
        let dn: InlineName = Default::default();

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
            "3om",
            "example.com",
            "sub.example.com",
            ".",
            "example.com.",
            "3xample.com",
            "EXAMPLE.com",
            "EXAMPLE.COM",
            "EXAMPLE.COM.",
            dn_253.as_str(),
            dn_254.as_str(),
        ];

        for sc in success_cases {
            let dn = InlineName::from(sc).unwrap();
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
            "3o-",
            "co-",
            "example..com",
            "sub..example.com",
            "example-.com",
            "-xample.com",
            "examp|e.com",
            "exa\u{203C}ple.com",
            dn_255.as_str(),
        ];

        for fc in failure_cases {
            assert!(InlineName::from(fc).is_err())
        }
    }

    #[test]
    fn test_len() {
        let mut dn = InlineName::new();
        assert_eq!(dn.len(), 0);

        dn.append_label("example").unwrap();
        assert_eq!(dn.len(), 8);

        dn.append_label("com").unwrap();
        assert_eq!(dn.len(), 12);
    }

    #[test]
    fn test_append_label_too_long() {
        let l_63 = "a".repeat(63);
        let l_62 = "b".repeat(62);

        let mut dn = InlineName::new();

        dn.append_label(&l_63).unwrap();
        assert_eq!(dn.len(), 64);

        dn.append_label(&l_63).unwrap();
        assert_eq!(dn.len(), 128);

        dn.append_label(&l_63).unwrap();
        assert_eq!(dn.len(), 192);

        // test total size > 255
        {
            let mut dn = dn.clone();
            dn.append_label("small").unwrap();

            let res = dn.append_label(&l_63);
            assert!(
                matches!(res, Err(Error::DomainNameTooLong(s)) if s == dn.len() + l_63.len() + 1)
            );
        }

        // test total size == 255
        let res = dn.clone().append_label(&l_63);
        assert!(matches!(res, Err(Error::DomainNameTooLong(s)) if s == dn.len() + l_63.len() + 1));

        dn.append_label(&l_62).unwrap();
        assert_eq!(dn.len(), 255);
    }

    #[test]
    fn test_eq() {
        let dn1 = InlineName::from("example.com").unwrap();
        let dn2 = InlineName::from("EXAMPLE.COM").unwrap();
        let dn3 = InlineName::from("eXaMpLe.cOm").unwrap();

        assert_eq!(dn1, dn2);
        assert_eq!(dn1, dn3);
        assert_eq!(dn2, dn3);
    }

    #[test]
    fn test_neq() {
        let dn1 = InlineName::from("example.com").unwrap();
        let dn2 = InlineName::from("sub.example.com").unwrap();
        let dn3 = InlineName::from("Sub.examp1e.com").unwrap();

        assert_ne!(dn1, dn2);
        assert_ne!(dn1, dn3);
        assert_ne!(dn2, dn3);
    }

    #[test]
    fn test_eq_str() {
        let dn1 = InlineName::from("example.com").unwrap();
        let dn2 = InlineName::from("EXAMPLE.COM").unwrap();

        assert_eq!(dn1, "EXAMPLE.COM.");
        assert_eq!(dn1, "EXAMPLE.COM");

        assert_eq!(dn1, "eXaMpLe.cOm.");
        assert_eq!(dn2, "eXaMpLe.cOm");

        assert_eq!(dn2, "eXaMpLe.cOm");
        assert_eq!(dn2, "eXaMpLe.cOm.");

        assert_eq!(
            InlineName::from("sub.example.com").unwrap(),
            "sub.example.com."
        );
        assert_eq!(
            InlineName::from("sub.example.com.").unwrap(),
            "sub.example.com"
        );

        assert_eq!(InlineName::new(), "");
        assert_eq!(InlineName::root(), ".");
    }

    #[test]
    fn test_neq_str() {
        let dn1 = InlineName::from("example.com").unwrap();
        let dn2 = InlineName::from("sub.example.com").unwrap();

        assert_ne!(dn1, "sub.example.com");
        assert_ne!(dn1, "sub.example.com.");

        assert_ne!(dn1, "Sub.examp1e.com");
        assert_ne!(dn1, "Sub.examp1e.com.");

        assert_ne!(dn2, "Sub.examp1e.com");
        assert_ne!(dn2, "Sub.examp1e.com.");

        assert_ne!(InlineName::new(), ".");
        assert_ne!(InlineName::root(), "");
    }

    #[test]
    fn test_hash() {
        let dn = InlineName::from("example.com").unwrap();

        let mut s = HashSet::new();
        s.insert(dn);

        assert!(s.contains(&InlineName::from("example.com.").unwrap()));
        assert!(s.contains(&InlineName::from("eXaMpLe.COM").unwrap()));
        assert!(s.contains(&InlineName::from("EXAMPLE.COM").unwrap()));

        assert!(!s.contains(&InlineName::from("suB.Example.com.").unwrap()));
    }

    #[test]
    fn test_ord() {
        let dn1 = InlineName::from("example.com").unwrap();
        let dn2 = InlineName::from("ExaMplE.com").unwrap();
        let dn3 = InlineName::from("Sub.example.com").unwrap();

        assert_eq!(Ordering::Equal, dn1.cmp(&dn2));
        assert_eq!(Ordering::Less, dn1.cmp(&dn3));
        assert_eq!(Ordering::Greater, dn3.cmp(&dn1));
        assert_eq!(Ordering::Equal, InlineName::root().cmp(&InlineName::root()));
        assert_eq!(Ordering::Equal, InlineName::new().cmp(&InlineName::new()));
    }

    #[test]
    fn test_partial_ord() {
        let dn1 = InlineName::from("example.com").unwrap();
        let dn2 = InlineName::from("ExaMplE.com").unwrap();
        let dn3 = InlineName::from("Sub.example.com").unwrap();

        assert_eq!(Some(Ordering::Equal), dn1.partial_cmp(&dn2));
        assert_eq!(Some(Ordering::Less), dn1.partial_cmp(&dn3));
        assert_eq!(Some(Ordering::Greater), dn3.partial_cmp(&dn1));
        assert_eq!(
            Some(Ordering::Equal),
            InlineName::root().partial_cmp(&InlineName::root())
        );
        assert_eq!(
            Some(Ordering::Equal),
            InlineName::new().partial_cmp(&InlineName::new())
        );
    }
}
