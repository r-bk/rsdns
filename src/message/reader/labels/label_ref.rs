/// An encoded domain name label returned from [`Labels`] iterator.
///
/// `LabelRef` doesn't own the label bytes, but rather points into a message buffer.
///
/// [`Labels`]: super::Labels
#[derive(Debug, Clone)]
pub struct LabelRef<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) pos: usize,
}

impl<'a> LabelRef<'a> {
    /// Returns the label bytes.
    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }
}
