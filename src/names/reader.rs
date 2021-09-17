use crate::{
    bytes::{Cursor, Reader},
    message::reader::{read_domain_name, skip_domain_name},
    names::{InlineName, Name},
    Result,
};

pub struct DomainNameReader;

impl DomainNameReader {
    #[inline]
    pub fn read(cursor: &mut Cursor<'_>) -> Result<InlineName> {
        read_domain_name(cursor)
    }

    #[inline]
    pub fn read_string(cursor: &mut Cursor<'_>) -> Result<Name> {
        read_domain_name(cursor)
    }

    #[inline]
    pub fn skip(cursor: &mut Cursor<'_>) -> Result<()> {
        skip_domain_name(cursor)
    }
}

impl Reader<InlineName> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> Result<InlineName> {
        DomainNameReader::read(self)
    }
}

impl Reader<Name> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> Result<Name> {
        DomainNameReader::read_string(self)
    }
}

impl Cursor<'_> {
    pub fn skip_domain_name(&mut self) -> Result<usize> {
        let start = self.pos();
        DomainNameReader::skip(self)?;
        Ok(self.pos() - start)
    }
}
