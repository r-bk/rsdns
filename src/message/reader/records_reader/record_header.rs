use crate::{
    constants::RecordsSection,
    message::{
        reader::{RecordMarker, RecordOffset},
        ClassValue, TypeValue,
    },
    names::DName,
};

/// A resource record header.
///
/// The difference between [`RecordHeader`] and [`RecordHeaderRef`] is that the former uses a domain
/// name type which owns the domain name bytes, while the latter uses [`NameRef`] which instead
/// is a reference to the original message buffer. Unfortunately, rust still doesn't allow
/// these two types to be expressed as a single type with a generic name parameter.
///
/// [`RecordsReader`]: super::RecordsReader
/// [`RecordHeaderRef`]: super::RecordHeaderRef
/// [`NameRef`]: crate::message::reader::NameRef
#[allow(dead_code)]
pub struct RecordHeader<N: DName> {
    pub(crate) name: N,
    pub(crate) marker: RecordMarker,
}

impl<N: DName> RecordHeader<N> {
    /// Returns the name of the record.
    #[inline]
    pub fn name(&self) -> &N {
        &self.name
    }

    /// Returns the record' marker.
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
