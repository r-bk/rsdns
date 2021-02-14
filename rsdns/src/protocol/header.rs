use crate::{
    protocol::{constants::HEADER_LENGTH, message::Cursor, Flags},
    Result, RsDnsError,
};

/// DNS message header.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[derive(Default)]
pub struct Header {
    /// An identifier assigned by the program that generates any kind of query.
    /// This identifier is copied to the corresponding reply and can be used by the requester to
    /// match up replies to outstanding requests.
    pub id: u16,
    /// Message flags.
    pub flags: Flags,
    /// Number of entries in the question section.
    pub qd_count: u16,
    /// Number of resource records in the answer section.
    pub an_count: u16,
    /// Number of name server resource records in the authority records section.
    pub ns_count: u16,
    /// Number of resource records in the additional records section.
    pub ar_count: u16,
}

impl Header {
    pub(crate) fn from_cursor(cursor: &mut Cursor) -> Result<Header> {
        if cursor.len() >= HEADER_LENGTH {
            unsafe {
                Ok(Header {
                    id: cursor.u16_be_unchecked(),
                    flags: Flags::new(cursor.u16_be_unchecked()),
                    qd_count: cursor.u16_be_unchecked(),
                    an_count: cursor.u16_be_unchecked(),
                    ns_count: cursor.u16_be_unchecked(),
                    ar_count: cursor.u16_be_unchecked(),
                })
            }
        } else {
            Err(RsDnsError::EndOfBuffer)
        }
    }
}
