use crate::{
    constants::{RClass, RType},
    records::data::RecordData,
    InlineName,
};

/// A resource record.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ResourceRecord {
    /// A domain name to which this resource record pertains.
    pub name: InlineName,

    /// The record class.
    pub rclass: RClass,

    /// The record type.
    pub rtype: RType,

    /// The time (in seconds) that the resource record may be cached before it should
    /// be discarded. Zero value means the record should not be cached.
    pub ttl: u32,

    /// The record data.
    pub rdata: RecordData,
}
