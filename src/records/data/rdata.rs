use crate::constants::RType;
use std::{fmt::Debug, hash::Hash};

pub(super) mod private {
    use crate::{errors::Result, records::data::RecordData};

    pub trait RDataBase {
        fn from(rd: RecordData) -> Result<Self>
        where
            Self: Sized;
    }
}

/// A marker trait for all record-data types.
pub trait RData:
    private::RDataBase + Clone + Eq + PartialEq + Hash + Debug + Ord + PartialOrd
{
    /// Record data type as associated constant.
    const RTYPE: RType;
}