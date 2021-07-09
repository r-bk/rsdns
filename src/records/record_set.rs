use crate::{
    constants::{RClass, RType},
    records::data::RecordData,
    Name,
};

/// A set of similar records.
///
/// RecordSet (or RRset) is a set of resource records with the same name, class and type,
/// but with different data.
///
/// Defined in [RFC 7719 section 4](https://datatracker.ietf.org/doc/html/rfc7719#section-4).
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RecordSet {
    /// The name all records in this set belong to.
    pub name: Name,

    /// The class of records in this set.
    pub rclass: RClass,

    /// The type of records in this set.
    pub rtype: RType,

    /// The TTL of records in this set.
    ///
    /// In case a DNS message contains records with different TTL, this is the minimum among them.
    pub ttl: u32,

    /// The various record data of this set.
    pub rdata: Vec<RecordData>,
}
