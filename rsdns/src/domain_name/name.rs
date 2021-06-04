use crate::{constants::DOMAIN_NAME_MAX_LENGTH, DomainNameArrayString, Error, Result};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
    str::FromStr,
};

/// A domain name backed by [String].
///
/// This struct implements the domain name using the standard [String].
/// Construction of [Name] involves dynamic memory allocation.
///
/// [Name] is used in resource record data, where usage of
/// [DomainNameArrayString](crate::DomainNameArrayString) would make the size of the structure
/// too large.
/// For example, the [Soa](crate::records::Soa) record includes two domain names in the record data.
/// This, together with the domain name in the record header, would make the size of the structure
/// at least 765 bytes long if [DomainNameArrayString](crate::DomainNameArrayString) was used.
///
/// [Name] stores the name in the canonical form `example.com.`.
/// The trailing period denotes the root DNS zone.
///
/// Domain name max length, as defined in
/// [RFC 1035](https://tools.ietf.org/html/rfc1035#section-3.1), is 255 bytes.
/// This includes all label length bytes, and the terminating zero length byte. Hence the effective
/// max length of a domain name without the root zone is 253 bytes.
///
/// Domain name is case insensitive. Hence, when compared, both sides are converted to
/// ASCII lowercase. Use [`Name::as_str`] when exact match is required.
///
/// Specifications:
///
/// - [RFC 1035 ~2.3.1](https://tools.ietf.org/html/rfc1035#section-2.3.1)
/// - [RFC 1035 ~2.3.4](https://tools.ietf.org/html/rfc1035#section-2.3.4)
/// - [RFC 1035 ~3.1](https://tools.ietf.org/html/rfc1035#section-3.1)
#[derive(Debug, Default, Clone)]
pub struct Name {
    str_: String,
}

impl Name {
    /// Creates an empty domain name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::Name;
    /// #
    /// let dn = Name::new();
    /// assert_eq!(dn.len(), 0);
    /// assert!(dn.is_empty());
    /// ```
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            str_: Default::default(),
        }
    }

    /// Creates the root domain name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::Name;
    /// #
    /// let dn = Name::new_root();
    /// assert_eq!(dn.len(), 1);
    /// assert_eq!(dn.as_str(), ".");
    /// ```
    pub fn new_root() -> Self {
        Self {
            str_: String::from("."),
        }
    }

    fn from(s: &str) -> Result<Self> {
        super::check_name(s)?;

        let mut dn = Self {
            // check_name verifies the length of the string,
            // so the following unwrap will not panic.
            str_: String::from(s),
        };

        let bytes = s.as_bytes();

        // check_name rejects an empty string, so it is sound to use unchecked access here
        let last_byte = unsafe { *bytes.get_unchecked(bytes.len() - 1) };

        if last_byte != b'.' {
            // check_name verifies the length of the string and ensures that
            // the root zone can be accommodated.
            // Thus the following push is sound and will not panic.
            dn.str_.push('.');
        }

        Ok(dn)
    }

    /// Returns the domain name as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::Name;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let dn = Name::new();
    /// assert_eq!(dn.as_str(), "");
    ///
    /// let dn = Name::from_str("example.com")?;
    /// assert_eq!(dn.as_str(), "example.com.");
    ///
    /// let dn = Name::from_str(".")?;
    /// assert_eq!(dn.as_str(), ".");
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.str_
    }

    /// Returns the length of the domain name in bytes.
    ///
    /// Valid domain names are comprised of ASCII characters only.
    /// Thus this value equals the number of characters in the domain name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::Name;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let dn = Name::new();
    /// assert_eq!(dn.len(), 0);
    ///
    /// let dn = Name::from_str("example.com")?;
    /// assert_eq!(dn.len(), 12); // includes the root zone
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.str_.len()
    }

    /// Checks if domain name is empty.
    ///
    /// **Note**: empty domain name is not valid.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::Name;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let dn = Name::from_str("example.com")?;
    /// assert_eq!(dn.is_empty(), false);
    ///
    /// let dn = Name::new();
    /// assert_eq!(dn.is_empty(), true);
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.str_.is_empty()
    }

    /// Make the domain name empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::Name;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let mut dn = Name::from_str("example.com")?;
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
        self.str_.clear();
    }

    /// Appends a label to the domain name.
    ///
    /// This function is dedicated to a parser which needs to construct
    /// a domain name label by label, as they are read from the DNS on-wire representation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::Name;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let mut dn = Name::new();
    ///
    /// dn.push_label_bytes(b"example")?;
    /// assert_eq!(dn.as_str(), "example.");
    ///
    /// dn.push_label_bytes(b"com")?;
    /// assert_eq!(dn.as_str(), "example.com.");
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    pub fn push_label_bytes(&mut self, label: &[u8]) -> Result<()> {
        super::check_label_bytes(label)?;

        // at this point the label is proven to be valid,
        // which means it is sound to convert it unchecked as a valid label is ASCII
        let label_as_str = unsafe { std::str::from_utf8_unchecked(label) };

        if self.str_.len() + label_as_str.len() + 1 > DOMAIN_NAME_MAX_LENGTH {
            return Err(Error::DomainNameTooLong);
        }

        self.str_.push_str(label_as_str);
        self.str_.push('.');

        Ok(())
    }

    /// Appends a label to the domain name.
    ///
    /// This is a string slice equivalent of [`Name::push_label_bytes`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::Name;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let mut dn = Name::new();
    ///
    /// dn.push_label("example")?;
    /// assert_eq!(dn.as_str(), "example.");
    ///
    /// dn.push_label("com")?;
    /// assert_eq!(dn.as_str(), "example.com.");
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    pub fn push_label(&mut self, label: &str) -> Result<()> {
        super::check_label(label)?;

        if self.str_.len() + label.len() + 1 > DOMAIN_NAME_MAX_LENGTH {
            return Err(Error::DomainNameTooLong);
        }

        self.str_.push_str(label);
        self.str_.push('.');

        Ok(())
    }

    /// Sets the domain name to denote the root DNS zone `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::Name;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let mut dn = Name::new();
    /// assert!(dn.is_empty());
    ///
    /// dn.set_root();
    /// assert_eq!(dn.as_str(), ".");
    ///
    /// dn = Name::from_str("example.com")?;
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
        self.str_.clear();
        self.str_.push('.');
    }
}

impl TryFrom<&str> for Name {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from(value)
    }
}

impl FromStr for Name {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from(s)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.str_
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.str_
            .as_bytes()
            .eq_ignore_ascii_case(other.str_.as_bytes())
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..self.len().min(other.len()) {
            let left = unsafe { self.str_.as_bytes().get_unchecked(i) };
            let right = unsafe { other.str_.as_bytes().get_unchecked(i) };
            let ord = left.to_ascii_lowercase().cmp(&right.to_ascii_lowercase());
            if Ordering::Equal != ord {
                return ord;
            }
        }
        self.len().cmp(&other.len())
    }
}

impl PartialEq<&str> for Name {
    fn eq(&self, other: &&str) -> bool {
        let l_is_root = self.str_.as_bytes() == b".";
        let r_is_root = *other == ".";

        match (l_is_root, r_is_root) {
            (true, true) => return true,
            (false, false) => {}
            _ => return false,
        }

        let mut bytes = self.str_.as_bytes();
        if !bytes.is_empty() && !other.ends_with('.') {
            bytes = &bytes[..bytes.len() - 1];
        }

        bytes.eq_ignore_ascii_case(other.as_bytes())
    }
}

impl Eq for Name {}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for b in self.str_.as_bytes() {
            state.write_u8(b.to_ascii_lowercase());
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<Name> for String {
    fn from(name: Name) -> Self {
        name.str_
    }
}

impl From<DomainNameArrayString> for Name {
    fn from(name: DomainNameArrayString) -> Self {
        Self {
            str_: name.as_str().to_string(),
        }
    }
}

impl super::NameContract for Name {
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
    fn push_label_bytes(&mut self, label: &[u8]) -> Result<()> {
        self.push_label_bytes(label)
    }

    #[inline(always)]
    fn push_label(&mut self, label: &str) -> Result<()> {
        self.push_label(label)
    }

    #[inline(always)]
    fn set_root(&mut self) {
        self.set_root()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_new() {
        let dn = Name::new();

        assert!(dn.is_empty());
        assert_eq!(dn.len(), 0);
    }

    #[test]
    fn test_default() {
        let dn: Name = Default::default();

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
            let dn = Name::from(sc).unwrap();
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
            assert!(Name::from(fc).is_err())
        }
    }

    #[test]
    fn test_len() {
        let mut dn = Name::new();
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

        let mut dn = Name::new();

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
        let dn1 = Name::from("example.com").unwrap();
        let dn2 = Name::from("EXAMPLE.COM").unwrap();
        let dn3 = Name::from("eXaMpLe.cOm").unwrap();

        assert_eq!(dn1, dn2);
        assert_eq!(dn1, dn3);
        assert_eq!(dn2, dn3);
    }

    #[test]
    fn test_neq() {
        let dn1 = Name::from("example.com").unwrap();
        let dn2 = Name::from("sub.example.com").unwrap();
        let dn3 = Name::from("Sub.examp1e.com").unwrap();

        assert_ne!(dn1, dn2);
        assert_ne!(dn1, dn3);
        assert_ne!(dn2, dn3);
    }

    #[test]
    fn test_eq_str() {
        let dn1 = Name::from("example.com").unwrap();
        let dn2 = Name::from("EXAMPLE.COM").unwrap();

        assert_eq!(dn1, "EXAMPLE.COM.");
        assert_eq!(dn1, "EXAMPLE.COM");

        assert_eq!(dn1, "eXaMpLe.cOm.");
        assert_eq!(dn2, "eXaMpLe.cOm");

        assert_eq!(dn2, "eXaMpLe.cOm");
        assert_eq!(dn2, "eXaMpLe.cOm.");

        assert_eq!(Name::from("sub.example.com").unwrap(), "sub.example.com.");
        assert_eq!(Name::from("sub.example.com.").unwrap(), "sub.example.com");

        assert_eq!(Name::new(), "");
        assert_eq!(Name::new_root(), ".");
    }

    #[test]
    fn test_neq_str() {
        let dn1 = Name::from("example.com").unwrap();
        let dn2 = Name::from("sub.example.com").unwrap();

        assert_ne!(dn1, "sub.example.com");
        assert_ne!(dn1, "sub.example.com.");

        assert_ne!(dn1, "Sub.examp1e.com");
        assert_ne!(dn1, "Sub.examp1e.com.");

        assert_ne!(dn2, "Sub.examp1e.com");
        assert_ne!(dn2, "Sub.examp1e.com.");

        assert_ne!(Name::new(), ".");
        assert_ne!(Name::new_root(), "");
    }

    #[test]
    fn test_hash() {
        let dn = Name::from("example.com").unwrap();

        let mut s = HashSet::new();
        s.insert(dn);

        assert!(s.contains(&Name::from("example.com.").unwrap()));
        assert!(s.contains(&Name::from("eXaMpLe.COM").unwrap()));
        assert!(s.contains(&Name::from("EXAMPLE.COM").unwrap()));

        assert!(!s.contains(&Name::from("suB.Example.com.").unwrap()));
    }

    #[test]
    fn test_ord() {
        let dn1 = Name::from("example.com").unwrap();
        let dn2 = Name::from("ExaMplE.com").unwrap();
        let dn3 = Name::from("Sub.example.com").unwrap();

        assert_eq!(Ordering::Equal, dn1.cmp(&dn2));
        assert_eq!(Ordering::Less, dn1.cmp(&dn3));
        assert_eq!(Ordering::Greater, dn3.cmp(&dn1));
        assert_eq!(Ordering::Equal, Name::new_root().cmp(&Name::new_root()));
        assert_eq!(Ordering::Equal, Name::new().cmp(&Name::new()));
    }

    #[test]
    fn test_partial_ord() {
        let dn1 = Name::from("example.com").unwrap();
        let dn2 = Name::from("ExaMplE.com").unwrap();
        let dn3 = Name::from("Sub.example.com").unwrap();

        assert_eq!(Some(Ordering::Equal), dn1.partial_cmp(&dn2));
        assert_eq!(Some(Ordering::Less), dn1.partial_cmp(&dn3));
        assert_eq!(Some(Ordering::Greater), dn3.partial_cmp(&dn1));
        assert_eq!(
            Some(Ordering::Equal),
            Name::new_root().partial_cmp(&Name::new_root())
        );
        assert_eq!(Some(Ordering::Equal), Name::new().partial_cmp(&Name::new()));
    }
}
