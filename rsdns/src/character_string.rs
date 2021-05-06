use crate::constants::CHARACTER_STRING_MAX_LENGTH;

type ArrType = arrayvec::ArrayVec<u8, CHARACTER_STRING_MAX_LENGTH>;

/// A character string.
///
/// Is treated as binary data.
///
/// [`RFC 1035 ~3.3`](https://tools.ietf.org/html/rfc1035#section-3.3)
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CharacterString {
    pub(crate) arr: ArrType,
}

impl CharacterString {
    /// Returns the length of the string.
    #[inline]
    pub fn len(&self) -> usize {
        self.arr.len()
    }

    /// Checks if the string is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the capacity of the underlying buffer.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.arr.capacity()
    }

    /// Clears the string making it empty.
    #[inline]
    pub fn clear(&mut self) {
        self.arr.clear();
    }

    /// Returns the byte slice of the string.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.arr.as_slice()
    }
}
