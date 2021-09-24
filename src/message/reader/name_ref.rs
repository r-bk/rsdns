use crate::{
    bytes::Cursor,
    message::reader::{read_domain_name, Labels},
    names::{InlineName, Name},
    Error, Result,
};
use std::convert::TryFrom;

/// An encoded domain name.
///
/// `NameRef` represents an encoded domain name. It doesn't hold the name bytes itself, but rather
/// points into a message buffer.
///
/// Its main purpose is efficient comparison between domain names encoded in the **same** DNS
/// message. This is optimized for compressed domain names, where two labels encoded at the same
/// offset ensure that suffixes of a domain name starting at these labels are equal.
///
/// `NameRef` doesn't implement the `PartialEq` trait, because that trait is infallible.
#[derive(Debug, Clone)]
pub struct NameRef<'a> {
    cursor: Cursor<'a>,
}

impl<'a> NameRef<'a> {
    #[allow(dead_code)]
    #[inline]
    pub(crate) fn new(cursor: Cursor<'a>) -> NameRef<'a> {
        NameRef { cursor }
    }

    /// Returns an iterator over the labels of this domain name.
    #[inline]
    pub fn labels(&self) -> Labels<'a> {
        Labels::new(self.cursor.clone())
    }

    /// Checks if this `NameRef` points to the same name as another one.
    ///
    /// This method is well defined only when applied to names encoded in the **same** DNS message.
    pub fn eq(&self, other: &Self) -> Result<bool> {
        let mut my_labels = self.labels();
        let mut other_labels = other.labels();

        loop {
            let mo = my_labels.next();
            let oo = other_labels.next();
            match (mo, oo) {
                (Some(mr), Some(or)) => {
                    // unwrap the labels
                    let ml = mr?;
                    let ol = or?;

                    // efficiently compare labels: if two labels are at the same offset,
                    // the rest of the domain name is necessary equal.
                    if ml.pos == ol.pos {
                        break Ok(true);
                    } else if !ml.bytes.eq_ignore_ascii_case(ol.bytes) {
                        break Ok(false);
                    } // else continue
                }
                (None, None) => break Ok(true),
                _ => break Ok(false),
            }
        }
    }

    /// Checks if this `NameRef` and another one point to different names.
    ///
    /// This method is the inverse of [`NameRef::eq`].
    ///
    /// This method is well defined only when applied to names encoded in the **same** DNS message.
    #[inline]
    pub fn ne(&self, other: &Self) -> Result<bool> {
        Ok(!self.eq(other)?)
    }
}

impl TryFrom<NameRef<'_>> for Name {
    type Error = Error;

    #[inline]
    fn try_from(mut value: NameRef<'_>) -> Result<Self> {
        read_domain_name(&mut value.cursor)
    }
}

impl TryFrom<&NameRef<'_>> for Name {
    type Error = Error;

    #[inline]
    fn try_from(value: &NameRef<'_>) -> Result<Self> {
        read_domain_name(&mut value.cursor.clone())
    }
}

impl TryFrom<NameRef<'_>> for InlineName {
    type Error = Error;

    #[inline]
    fn try_from(mut value: NameRef<'_>) -> Result<Self> {
        read_domain_name(&mut value.cursor)
    }
}

impl TryFrom<&NameRef<'_>> for InlineName {
    type Error = Error;

    #[inline]
    fn try_from(value: &NameRef<'_>) -> Result<Self> {
        read_domain_name(&mut value.cursor.clone())
    }
}
