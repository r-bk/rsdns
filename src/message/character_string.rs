use crate::{bytes::Cursor, ProtocolResult};

impl Cursor<'_> {
    pub fn read_character_string(&mut self) -> ProtocolResult<Vec<u8>> {
        let len = self.u8()?;
        Ok(Vec::from(self.slice(len as usize)?))
    }
}
