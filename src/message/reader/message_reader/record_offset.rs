/// A resource record offset within a DNS message.
///
/// Note that comparison of `RecordOffset` is defined only when compared to an offset taken from the
/// same DNS message.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RecordOffset {
    /// name offset (variable length field)
    pub(crate) offset: usize,
    /// type offset (the first field after name)
    pub(crate) type_offset: usize,
}
