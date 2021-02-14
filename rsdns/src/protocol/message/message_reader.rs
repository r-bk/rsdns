use crate::{
    protocol::{
        constants::HEADER_LENGTH,
        message::{Cursor, DomainNameReader, QuestionsReader},
        Header,
    },
    Result,
};

/// A set of low-level primitives for parsing a raw DNS message.
#[allow(dead_code)]
pub struct MessageReader<'a> {
    buf: &'a [u8],
    header: Header,
    an_offset: usize,
}

impl<'a> MessageReader<'a> {
    /// Creates a `MessageReader` for a DNS message contained in `buf`.
    pub fn new(buf: &'a [u8]) -> Result<MessageReader<'a>> {
        let mut cursor = Cursor::new(buf);
        let header = Header::from_cursor(&mut cursor)?;
        let an_offset = Self::find_an_offset(cursor, header.qd_count as usize)?;
        Ok(MessageReader {
            buf,
            header,
            an_offset,
        })
    }

    /// Returns the parsed DNS message header.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Returns a reader for the questions section of the DNS message.
    pub fn questions(&self) -> QuestionsReader {
        QuestionsReader::new(
            Cursor::with_pos(self.buf, HEADER_LENGTH),
            self.header.qd_count,
        )
    }

    /// Finds the offset of the answers section.
    ///
    /// Skips the questions section.
    fn find_an_offset(mut cursor: Cursor, qd_count: usize) -> Result<usize> {
        for _ in 0..qd_count {
            DomainNameReader::skip(&mut cursor)?;
            cursor.advance(4)?; // qtype(2) + qclass(2)
        }

        Ok(cursor.pos())
    }
}
