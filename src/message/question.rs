use crate::{
    bytes::{Cursor, Reader},
    message::{ClassValue, TypeValue},
    names::InlineName,
    Result,
};

/// Query question.
///
/// Questions appear in the questions section of a query to carry the parameters that define
/// what is being asked. Usually only a single question is carried in a query, and it is
/// copied to the response as is.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Question {
    /// Domain name to query.
    pub qname: InlineName,
    /// Question type.
    pub qtype: TypeValue,
    /// Question class.
    pub qclass: ClassValue,
}

impl Reader<Question> for Cursor<'_> {
    fn read(&mut self) -> Result<Question> {
        Ok(Question {
            qname: self.read()?,
            qtype: self.read()?,
            qclass: self.read()?,
        })
    }
}
