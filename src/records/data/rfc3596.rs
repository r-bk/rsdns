use crate::{
    bytes::{Cursor, Reader, RrDataReader},
    records::Type,
    Result,
};
use std::net::Ipv6Addr;

/// A host address (IPv6).
///
/// [RFC 3596 section 2.2](https://www.rfc-editor.org/rfc/rfc3596.html#section-2.2)
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Aaaa {
    /// Internet address (IPv6).
    pub address: Ipv6Addr,
}

rr_data!(Aaaa, Type::AAAA);

impl RrDataReader<Aaaa> for Cursor<'_> {
    fn read_rr_data(&mut self, rd_len: usize) -> Result<Aaaa> {
        self.window(rd_len)?;
        let rr = Ok(Aaaa {
            address: self.read()?,
        });
        self.close_window()?;
        rr
    }
}
