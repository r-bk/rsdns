use crate::protocol::Flags;

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
