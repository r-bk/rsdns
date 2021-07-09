use crate::{
    bytes::{Cursor, Reader},
    constants::QClass,
    message::RecordType,
    InlineName, Result,
};
use std::convert::TryFrom;

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
    pub qclass: QClass,
}

impl Question {
    pub(crate) fn read(cursor: &mut Cursor) -> Result<Question> {
        Ok(Question {
            qname: cursor.read()?,
            qtype: cursor.read()?,
            qclass: QClass::try_from(cursor.u16_be()?)?,
        })
    }
}
