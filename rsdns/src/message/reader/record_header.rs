use crate::{
    constants::RecordsSection,
    message::{RecordClass, RecordType},
    InlineName,
};

/// Record header returned from [RecordsReader::peek_header].
///
/// [RecordsReader::peek_header]: crate::message::reader::RecordsReader::peek_header
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RecordHeader {
    // Cursor position this header was read from
    pub(crate) pos: usize,
    pub(crate) section: RecordsSection,
    pub(crate) name: InlineName,
    pub(crate) name_end_pos: usize,
    pub(crate) rtype: RecordType,
    pub(crate) rclass: RecordClass,
    pub(crate) ttl: u32,
    pub(crate) rdlen: u16,
}

impl RecordHeader {
    /// Returns the section this record belongs to.
    #[inline]
    pub fn section(&self) -> RecordsSection {
        self.section
    }

    /// Returns the domain name this record belongs to.
    #[inline]
    pub fn name(&self) -> &InlineName {
        &self.name
    }

    /// Returns the record type.
    #[inline]
    pub fn rtype(&self) -> RecordType {
        self.rtype
    }

    /// Returns the record class.
    #[inline]
    pub fn rclass(&self) -> RecordClass {
        self.rclass
    }

    /// Returns the record TTL.
    #[inline]
    pub fn ttl(&self) -> u32 {
        self.ttl
    }
}
