use crate::{
    constants::{Class, Type},
    names::InlineName,
    records::data::RecordData,
};

/// A resource record.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ResourceRecord {
    /// A domain name to which this resource record pertains.
    pub name: InlineName,

    /// The record class.
    pub rclass: Class,

    /// The record type.
    pub rtype: Type,

    /// The time (in seconds) that the resource record may be cached before it should
    /// be discarded. Zero value means the record should not be cached.
    pub ttl: u32,

    /// The record data.
    pub rdata: RecordData,
}
