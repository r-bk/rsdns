/// An encoded domain name label returned from [`Labels`] iterator.
///
/// [`Labels`]: super::Labels
#[derive(Debug)]
pub struct Label<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) pos: usize,
}

impl<'a> Label<'a> {
    /// Returns the label bytes.
    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }
}

impl<'a> PartialEq<Label<'a>> for Label<'a> {
    fn eq(&self, other: &Label<'a>) -> bool {
        if self.pos == other.pos {
            return true;
        }
        self.bytes.eq_ignore_ascii_case(other.bytes)
    }
}

impl Eq for Label<'_> {}
