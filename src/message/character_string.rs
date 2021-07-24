use crate::{bytes::Cursor, Result};

impl Cursor<'_> {
    pub fn read_character_string(&mut self) -> Result<Vec<u8>> {
        let len = self.u8()?;
        Ok(Vec::from(self.slice(len as usize)?))
    }
}
