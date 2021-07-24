use crate::constants::RType;
use std::{fmt::Debug, hash::Hash};

mod private {
    pub trait RDataBase {}
}

/// A marker trait for all record-data types.
pub trait RData:
    private::RDataBase + Clone + Eq + PartialEq + Hash + Default + Debug + Ord + PartialOrd
{
    /// Record data type as associated constant.
    const RTYPE: RType;
}
