use crate::{
    constants::RecordsSection,
    message::{
        reader::{NameRef, RecordMarker, RecordOffset},
        ClassValue, TypeValue,
    },
};

/// A resource record header with [`NameRef`].
///
/// [`RecordHeaderRef`] is identical to [`RecordHeader`] except for the type used for the domain
/// name. It uses [`NameRef`] which doesn't hold the domain name bytes itself, but rather points
/// back to the message buffer. This allows parsing a DNS message while avoiding unnecessary
/// memory allocations for the domain name (for example in the process of CNAME flattening).
///
/// Ideally,
///
/// [`RecordsReader`]: super::RecordsReader
/// [`RecordHeader`]: super::RecordHeader
#[allow(dead_code)]
pub struct RecordHeaderRef<'a> {
    pub(crate) name_ref: NameRef<'a>,
    pub(crate) marker: RecordMarker,
}

impl<'a> RecordHeaderRef<'a> {
    /// Returns the name of the record.
    #[inline]
    pub fn name(&self) -> &NameRef<'a> {
        &self.name_ref
    }

    /// Returns the record's marker.
    #[inline]
    pub fn marker(&self) -> &RecordMarker {
        &self.marker
    }

    /// Returns the record's offset.
    #[inline]
    pub fn offset(&self) -> RecordOffset {
        self.marker.offset
    }

    /// Returns the record's Type.
    #[inline]
    pub fn rtype(&self) -> TypeValue {
        self.marker.rtype
    }

    /// Returns the record's Class.
    #[inline]
    pub fn rclass(&self) -> ClassValue {
        self.marker.rclass
    }

    /// Returns the record's TTL.
    #[inline]
    pub fn ttl(&self) -> u32 {
        self.marker.ttl
    }

    /// Returns the record's data length.
    #[inline]
    pub fn rdlen(&self) -> u16 {
        self.marker.rdlen
    }

    /// Returns the record's section.
    #[inline]
    pub fn section(&self) -> RecordsSection {
        self.marker.section
    }
}
