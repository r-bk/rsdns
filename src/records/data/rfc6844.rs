use crate::{
    Result,
    bytes::{Cursor, RrDataReader},
    records::Type,
};

/// Certification Authority Authorization.
///
/// [RFC 6844](https://www.rfc-editor.org/rfc/rfc6844.html)
/// (obsoleted by [RFC 8659](https://www.rfc-editor.org/rfc/rfc8659.html);
/// the wire format is unchanged).
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
pub struct Caa {
    /// Flags octet. Bit 0 (mask `0x80`) is the Issuer Critical flag;
    /// the other bits are reserved.
    pub flags: u8,

    /// The property tag, e.g. `issue`, `issuewild`, `iodef`.
    /// ASCII alphanumeric, 1–15 octets in practice.
    pub tag: Vec<u8>,

    /// The property value. Format depends on `tag`.
    pub value: Vec<u8>,
}

rr_data!(Caa, Type::CAA);

impl RrDataReader<Caa> for Cursor<'_> {
    fn read_rr_data(&mut self, rd_len: usize) -> Result<Caa> {
        self.window(rd_len)?;
        let flags = self.u8()?;
        let tag_len = self.u8()? as usize;
        let tag = self.slice(tag_len)?.to_vec();
        let value_len = self.len();
        let value = self.slice(value_len)?.to_vec();
        self.close_window()?;
        Ok(Caa { flags, tag, value })
    }
}
