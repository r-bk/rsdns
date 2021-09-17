use crate::{
    bytes::Cursor,
    message::reader::{read_domain_name, skip_domain_name},
    names::{InlineName, Name},
    Result,
};

#[allow(dead_code)]
pub struct DomainNameReader;

#[allow(dead_code)]
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
    pub fn skip(cursor: &mut Cursor<'_>) -> Result<usize> {
        skip_domain_name(cursor)
    }
}
