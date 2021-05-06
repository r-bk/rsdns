use crate::{
    bytes::{Cursor, Reader},
    constants::HEADER_LENGTH,
    message::{
        reader::{Questions, Records},
        Header,
    },
    DomainNameReader, Result,
};

/// A DNS message reader.
#[allow(dead_code)]
pub struct MessageReader<'a> {
    buf: &'a [u8],
    header: Header,
    an_offset: usize,
}

impl<'a> MessageReader<'a> {
    /// Creates a `MessageReader` for a DNS message contained in `buf`.
    pub fn new(buf: &'a [u8]) -> Result<Self> {
        let mut cursor = Cursor::new(buf);
        let header: Header = cursor.read()?;
        let an_offset = Self::find_an_offset(cursor, header.qd_count as usize)?;
        Ok(MessageReader {
            buf,
            header,
            an_offset,
        })
    }

    /// Returns the parsed DNS header.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Returns an iterator over the questions section of the DNS message.
    pub fn questions(&self) -> Questions {
        Questions::new(
            Cursor::with_pos(self.buf, HEADER_LENGTH),
            self.header.qd_count,
        )
    }

    /// Returns an iterator over the resource record sections of the DNS message.
    pub fn records(&self) -> Records {
        Records::new(Cursor::with_pos(self.buf, self.an_offset), &self.header)
    }

    /// Finds the offset of the answers section.
    ///
    /// Skips the questions section.
    fn find_an_offset(mut cursor: Cursor, qd_count: usize) -> Result<usize> {
        for _ in 0..qd_count {
            DomainNameReader::skip(&mut cursor)?;
            cursor.skip(4)?; // qtype(2) + qclass(2)
        }

        Ok(cursor.pos())
    }
}
