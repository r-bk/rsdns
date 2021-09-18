use crate::{
    constants::RecordsSection,
    message::{reader::RecordOffset, ClassValue, TypeValue},
};

// The distance from the start of the TYPE field till the RDATA field.
const TYPE_TO_RDATA_OFFSET: usize = 10;

/// A resource record marker.
///
/// `RecordMarker` holds all the information about resource record except its data and the domain
/// name. It is used in [`RecordsReader`] to obtain additional information about a record, e.g. its
/// data.
///
/// Note that comparison of record markers is defined only between two markers obtained from
/// the **same** DNS message.
///
/// [`RecordsReader`]: super::RecordsReader
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RecordMarker {
    pub(crate) offset: RecordOffset,
    pub(crate) rtype: TypeValue,
    pub(crate) rclass: ClassValue,
    pub(crate) ttl: u32,
    pub(crate) rdlen: u16,
    pub(crate) section: RecordsSection,
}

impl RecordMarker {
    /// Returns the record's offset.
    #[inline]
    pub fn offset(&self) -> RecordOffset {
        self.offset
    }

    #[inline]
    pub(crate) fn rdata_pos(&self) -> usize {
        self.offset.type_offset + TYPE_TO_RDATA_OFFSET
    }

    /// Returns the record's Type.
    #[inline]
    pub fn rtype(&self) -> TypeValue {
        self.rtype
    }

    /// Returns the record's Class.
    #[inline]
    pub fn rclass(&self) -> ClassValue {
        self.rclass
    }

    /// Returns the record's TTL.
    #[inline]
    pub fn ttl(&self) -> u32 {
        self.ttl
    }

    /// Returns the record's data length.
    #[inline]
    pub fn rdlen(&self) -> u16 {
        self.rdlen
    }

    /// Returns the record's section.
    #[inline]
    pub fn section(&self) -> RecordsSection {
        self.section
    }
}
