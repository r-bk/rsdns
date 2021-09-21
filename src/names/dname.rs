use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub(super) mod private {
    use crate::{bytes::Cursor, Result};

    /// An interface of a domain name.
    pub trait DNameBase: Sized {
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
        fn append_label_bytes(&mut self, label: &[u8]) -> Result<()>;

        /// Appends a label to the domain name.
        fn append_label(&mut self, label: &str) -> Result<()>;

        /// Sets the domain name to denote the root DNS zone `.`.
        fn set_root(&mut self);

        /// Reads a domain name from a cursor.
        fn from_cursor(c: &mut Cursor<'_>) -> Result<Self>;
    }
}

/// A marker trait for all domain-name types.
pub trait DName:
    private::DNameBase + PartialOrd + Ord + PartialEq + Eq + FromStr + Debug + Display + Clone + Default
{
}
