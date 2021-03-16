use crate::{
    protocol::{bytes::Cursor, message::reader::DomainNameReader, DomainName, QClass, QType},
    Result,
};
use std::convert::TryFrom;

/// A DNS question record.
#[derive(Debug, Clone)]
pub struct Question {
    /// Domain name to query.
    pub qname: DomainName,
    /// Question type.
    pub qtype: QType,
    /// Question class.
    pub qclass: QClass,
}

impl Question {
    pub(crate) fn read(cursor: &mut Cursor) -> Result<Question> {
        Ok(Question {
            qname: DomainNameReader::read(cursor)?,
            qtype: QType::try_from(cursor.u16_be()?)?,
            qclass: QClass::try_from(cursor.u16_be()?)?,
        })
    }
}
