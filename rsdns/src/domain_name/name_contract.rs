use crate::ProtocolResult;
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

/// An interface of a domain name.
pub trait NameContract: PartialOrd + Ord + PartialEq + Eq + FromStr + Debug + Display {
    /// Returns the domain name as string slice.
    fn as_str(&self) -> &str;

    /// Returns the length of the domain name in bytes.
    ///
    /// Valid domain names are comprised of ASCII characters only.
    /// Thus this value equals the number of characters in the domain name.
    fn len(&self) -> usize;

    /// Checks of the domain name is empty.
    fn is_empty(&self) -> bool;

    /// Clears the domain name to be empty.
    fn clear(&mut self);

    /// Appends a label to the domain name.
    fn append_label_bytes(&mut self, label: &[u8]) -> ProtocolResult<()>;

    /// Appends a label to the domain name.
    fn append_label(&mut self, label: &str) -> ProtocolResult<()>;

    /// Sets the domain name to denote the root DNS zone `.`.
    fn set_root(&mut self);
}
