use crate::{
    Result,
    bytes::{Cursor, Reader},
    message::reader::NameRef,
    records::{Class, Type},
};

/// Query question with [`NameRef`].
///
/// As opposed to [`Question`], `QuestionRef` uses a `NameRef` for the domain name.
/// It doesn't own the domain name bytes, but rather points into the message buffer.
///
/// [`Question`]: crate::message::Question
#[derive(Debug, Clone)]
pub struct QuestionRef<'a> {
    /// Domain name to query.
    pub qname: NameRef<'a>,
    /// Question type.
    pub qtype: Type,
    /// Question class.
    pub qclass: Class,
}

impl<'a> Reader<QuestionRef<'a>> for Cursor<'a> {
    fn read(&mut self) -> Result<QuestionRef<'a>> {
        let qname = NameRef::new(self.clone());
        self.skip_domain_name()?;
        Ok(QuestionRef {
            qname,
            qtype: self.read()?,
            qclass: self.read()?,
        })
    }
}
