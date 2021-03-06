use crate::{
    bytes::{Cursor, Reader},
    message::{RecordClass, RecordType},
    InlineName, ProtocolResult,
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
    pub qtype: RecordType,
    /// Question class.
    pub qclass: RecordClass,
}

impl Reader<Question> for Cursor<'_> {
    fn read(&mut self) -> ProtocolResult<Question> {
        Ok(Question {
            qname: self.read()?,
            qtype: self.read()?,
            qclass: self.read()?,
        })
    }
}
