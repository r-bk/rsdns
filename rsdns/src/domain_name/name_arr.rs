use crate::{constants::DOMAIN_NAME_MAX_LENGTH, Error, Result};
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
/// It is used in cases when dynamic memory allocation is undesirable. In particular, [rsdns](crate)
/// uses it in resource record header. As a consequence parsing of resource records with no
/// variable size fields (e.g. [A](crate::records::A), [AAAA](crate::records::Aaaa)) involves
/// no memory allocation at all.
///
/// [DomainNameArr] stores the name in the canonical form `example.com.`.
/// The trailing period denotes the root DNS zone.
///
/// Domain name max length, as defined in
/// [RFC 1035](https://tools.ietf.org/html/rfc1035#section-3.1), is 255 bytes.
/// This includes all label length bytes, and the terminating zero length byte. Hence the effective
/// max length of a domain name without the root zone is 253 bytes.
///
/// Domain name is case insensitive. Hence, when compared, both sides are converted to
/// ASCII lowercase. Use [`DomainNameArr::as_str`] when exact match is required.
///
/// Specifications:
///
/// - [RFC 1035 ~2.3.1](https://tools.ietf.org/html/rfc1035#section-2.3.1)
/// - [RFC 1035 ~2.3.4](https://tools.ietf.org/html/rfc1035#section-2.3.4)
/// - [RFC 1035 ~3.1](https://tools.ietf.org/html/rfc1035#section-3.1)
#[derive(Debug, Default, Clone)]
pub struct DomainNameArr {
    arr: ArrayType,
}

impl DomainNameArr {
    /// Creates an empty domain name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::DomainNameArr;
    /// #
    /// let dn = DomainNameArr::new();
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
    /// # use rsdns::DomainNameArr;
    /// #
    /// let dn = DomainNameArr::new_root();
    /// assert_eq!(dn.len(), 1);
    /// assert_eq!(dn.as_str(), ".");
    /// ```
    pub fn new_root() -> Self {
        let mut dn = Self::default();
        dn.set_root();
        dn
    }

    /// Creates a domain name from a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::DomainNameArr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// let dn = DomainNameArr::from("example.com")?;
    /// assert_eq!(dn.as_str(), "example.com.");
    ///
    /// let dn = DomainNameArr::from("sub.example.com.")?;
    /// assert_eq!(dn.as_str(), "sub.example.com.");
    /// #
    /// # Ok(())
    /// # }
    /// # foo().unwrap();
    /// ```
    pub fn from(s: &str) -> Result<Self> {
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
    /// # use rsdns::DomainNameArr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// let dn = DomainNameArr::new();
    /// assert_eq!(dn.as_str(), "");
    ///
    /// let dn = DomainNameArr::from("example.com")?;
    /// assert_eq!(dn.as_str(), "example.com.");
    ///
    /// let dn = DomainNameArr::from(".")?;
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
    /// # use rsdns::DomainNameArr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let dn = DomainNameArr::new();
    /// assert_eq!(dn.len(), 0);
    ///
    /// let dn = DomainNameArr::from("example.com")?;
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
    /// # use rsdns::DomainNameArr;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let dn = DomainNameArr::from_str("example.com")?;
    /// assert_eq!(dn.is_empty(), false);
    ///
    /// let dn = DomainNameArr::new();
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
    /// # use rsdns::DomainNameArr;
    /// # use std::str::FromStr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let mut dn = DomainNameArr::from_str("example.com")?;
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

    /// Appends a label to the domain name.
    ///
    /// This function is dedicated to a parser which needs to construct
    /// a domain name label by label, as they are read from the DNS on-wire representation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::DomainNameArr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let mut dn = DomainNameArr::new();
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

        if self.arr.try_push_str(label_as_str).is_err() {
            return Err(Error::DomainNameTooLong);
        }

        if self.arr.try_push('.').is_err() {
            return Err(Error::DomainNameTooLong);
        }

        Ok(())
    }

    /// Appends a label to the domain name.
    ///
    /// This is a string slice equivalent of [`DomainNameArr::push_label_bytes`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::DomainNameArr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let mut dn = DomainNameArr::new();
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

        if self.arr.try_push_str(label).is_err() {
            return Err(Error::DomainNameTooLong);
        }

        if self.arr.try_push('.').is_err() {
            return Err(Error::DomainNameTooLong);
        }

        Ok(())
    }

    /// Sets the domain name to denote the root DNS zone `.`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rsdns::DomainNameArr;
    /// #
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let mut dn = DomainNameArr::new();
    /// assert!(dn.is_empty());
    ///
    /// dn.set_root();
    /// assert_eq!(dn.as_str(), ".");
    ///
    /// dn = DomainNameArr::from("example.com")?;
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

impl TryFrom<&str> for DomainNameArr {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::from(value)
    }
}

impl FromStr for DomainNameArr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from(s)
    }
}

impl AsRef<str> for DomainNameArr {
    fn as_ref(&self) -> &str {
        self.arr.as_str()
    }
}

impl PartialEq for DomainNameArr {
    fn eq(&self, other: &Self) -> bool {
        self.arr
            .as_bytes()
            .eq_ignore_ascii_case(other.arr.as_bytes())
    }
}

impl PartialOrd for DomainNameArr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DomainNameArr {
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

impl PartialEq<&str> for DomainNameArr {
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

impl Eq for DomainNameArr {}

impl Hash for DomainNameArr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for b in self.arr.as_bytes() {
            state.write_u8(b.to_ascii_lowercase());
        }
    }
}

impl Display for DomainNameArr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl super::Name for DomainNameArr {
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
        let dn = DomainNameArr::new();

        assert!(dn.is_empty());
        assert_eq!(dn.len(), 0);
    }

    #[test]
    fn test_default() {
        let dn: DomainNameArr = Default::default();

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
            let dn = DomainNameArr::from(sc).unwrap();
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
            assert!(DomainNameArr::from(fc).is_err())
        }
    }

    #[test]
    fn test_len() {
        let mut dn = DomainNameArr::new();
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

        let mut dn = DomainNameArr::new();

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
        let dn1 = DomainNameArr::from("example.com").unwrap();
        let dn2 = DomainNameArr::from("EXAMPLE.COM").unwrap();
        let dn3 = DomainNameArr::from("eXaMpLe.cOm").unwrap();

        assert_eq!(dn1, dn2);
        assert_eq!(dn1, dn3);
        assert_eq!(dn2, dn3);
    }

    #[test]
    fn test_neq() {
        let dn1 = DomainNameArr::from("example.com").unwrap();
        let dn2 = DomainNameArr::from("sub.example.com").unwrap();
        let dn3 = DomainNameArr::from("Sub.examp1e.com").unwrap();

        assert_ne!(dn1, dn2);
        assert_ne!(dn1, dn3);
        assert_ne!(dn2, dn3);
    }

    #[test]
    fn test_eq_str() {
        let dn1 = DomainNameArr::from("example.com").unwrap();
        let dn2 = DomainNameArr::from("EXAMPLE.COM").unwrap();

        assert_eq!(dn1, "EXAMPLE.COM.");
        assert_eq!(dn1, "EXAMPLE.COM");

        assert_eq!(dn1, "eXaMpLe.cOm.");
        assert_eq!(dn2, "eXaMpLe.cOm");

        assert_eq!(dn2, "eXaMpLe.cOm");
        assert_eq!(dn2, "eXaMpLe.cOm.");

        assert_eq!(
            DomainNameArr::from("sub.example.com").unwrap(),
            "sub.example.com."
        );
        assert_eq!(
            DomainNameArr::from("sub.example.com.").unwrap(),
            "sub.example.com"
        );

        assert_eq!(DomainNameArr::new(), "");
        assert_eq!(DomainNameArr::new_root(), ".");
    }

    #[test]
    fn test_neq_str() {
        let dn1 = DomainNameArr::from("example.com").unwrap();
        let dn2 = DomainNameArr::from("sub.example.com").unwrap();

        assert_ne!(dn1, "sub.example.com");
        assert_ne!(dn1, "sub.example.com.");

        assert_ne!(dn1, "Sub.examp1e.com");
        assert_ne!(dn1, "Sub.examp1e.com.");

        assert_ne!(dn2, "Sub.examp1e.com");
        assert_ne!(dn2, "Sub.examp1e.com.");

        assert_ne!(DomainNameArr::new(), ".");
        assert_ne!(DomainNameArr::new_root(), "");
    }

    #[test]
    fn test_hash() {
        let dn = DomainNameArr::from("example.com").unwrap();

        let mut s = HashSet::new();
        s.insert(dn);

        assert!(s.contains(&DomainNameArr::from("example.com.").unwrap()));
        assert!(s.contains(&DomainNameArr::from("eXaMpLe.COM").unwrap()));
        assert!(s.contains(&DomainNameArr::from("EXAMPLE.COM").unwrap()));

        assert!(!s.contains(&DomainNameArr::from("suB.Example.com.").unwrap()));
    }

    #[test]
    fn test_ord() {
        let dn1 = DomainNameArr::from("example.com").unwrap();
        let dn2 = DomainNameArr::from("ExaMplE.com").unwrap();
        let dn3 = DomainNameArr::from("Sub.example.com").unwrap();

        assert_eq!(Ordering::Equal, dn1.cmp(&dn2));
        assert_eq!(Ordering::Less, dn1.cmp(&dn3));
        assert_eq!(Ordering::Greater, dn3.cmp(&dn1));
        assert_eq!(
            Ordering::Equal,
            DomainNameArr::new_root().cmp(&DomainNameArr::new_root())
        );
        assert_eq!(
            Ordering::Equal,
            DomainNameArr::new().cmp(&DomainNameArr::new())
        );
    }

    #[test]
    fn test_partial_ord() {
        let dn1 = DomainNameArr::from("example.com").unwrap();
        let dn2 = DomainNameArr::from("ExaMplE.com").unwrap();
        let dn3 = DomainNameArr::from("Sub.example.com").unwrap();

        assert_eq!(Some(Ordering::Equal), dn1.partial_cmp(&dn2));
        assert_eq!(Some(Ordering::Less), dn1.partial_cmp(&dn3));
        assert_eq!(Some(Ordering::Greater), dn3.partial_cmp(&dn1));
        assert_eq!(
            Some(Ordering::Equal),
            DomainNameArr::new_root().partial_cmp(&DomainNameArr::new_root())
        );
        assert_eq!(
            Some(Ordering::Equal),
            DomainNameArr::new().partial_cmp(&DomainNameArr::new())
        );
    }
}
