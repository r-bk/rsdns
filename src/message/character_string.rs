use crate::{
    bytes::{CSize, Cursor},
    Result,
};

impl Cursor<'_> {
    pub fn read_character_string(&mut self) -> Result<Vec<u8>> {
        let len = self.u8()?;
        Ok(Vec::from(self.slice(CSize(len as u16))?))
    }
}
