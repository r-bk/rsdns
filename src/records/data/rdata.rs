use crate::records::Type;
use std::{fmt::Debug, hash::Hash};

pub(super) mod private {
    use crate::{bytes::Cursor, errors::Result};

    pub trait RDataBase {
        fn from_cursor(c: &mut Cursor<'_>, rdlen: usize) -> Result<Self>
        where
            Self: Sized;
    }
}

/// A marker trait for all record-data types.
pub trait RData:
    private::RDataBase + Clone + Eq + PartialEq + Hash + Debug + Ord + PartialOrd
{
    /// Record data type as associated constant.
    const RTYPE: Type;
}
