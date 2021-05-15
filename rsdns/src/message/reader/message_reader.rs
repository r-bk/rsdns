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
///
/// MessageReader is the main utility for parsing messages.
///
/// Resource records are read from the message buffer with minimal amount of dynamic memory
/// allocation. Memory is allocated only for those records which contain variable size fields in the
/// record data section. In particular, reading A and AAAA records doesn't involve dynamic memory
/// allocation at all.
///
/// # Examples
///
/// ```rust
/// use rsdns::{
///     constants::MessageSection,
///     message::reader::MessageReader,
///     records::ResourceRecord,
/// };
///
/// fn read_message(buf: &[u8]) -> rsdns::Result<()> {
///     let mr = MessageReader::new(buf)?;
///
///     // mr.header() returns the parsed Header
///     // mr.questions() returns an iterator over the questions section
///
///     for record in mr.records() {
///         let (section, record) = record?;
///
///         if section != MessageSection::Answer {
///             // skip other sections
///             break;
///         }
///
///         match record {
///             ResourceRecord::A(ref rec) => {
///                 println!(
///                     "Name: {}; Class: {}; TTL: {}; ipv4: {}",
///                     rec.name, rec.rclass, rec.ttl, rec.data.address
///                 );
///             }
///             ResourceRecord::Aaaa(ref rec) => {
///                 println!(
///                     "Name: {}; Class: {}; TTL: {}; ipv6: {}",
///                     rec.name, rec.rclass, rec.ttl, rec.data.address
///                 );
///             }
///             _ => println!("{:?} {:?}", section, record),
///         }
///     }
///
///     Ok(())
/// }
///
/// ```
#[allow(dead_code)]
pub struct MessageReader<'a> {
    buf: &'a [u8],
    header: Header,
    an_offset: usize,
}

impl<'a> MessageReader<'a> {
    /// Creates a reader for a message contained in `buf`.
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

    /// Returns the parsed header.
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Returns an iterator over the questions section of the message.
    pub fn questions(&self) -> Questions {
        Questions::new(
            Cursor::with_pos(self.buf, HEADER_LENGTH),
            self.header.qd_count,
        )
    }

    /// Returns an iterator over the resource record sections of the message.
    pub fn records(&self) -> Records {
        Records::new(Cursor::with_pos(self.buf, self.an_offset), &self.header)
    }

    fn find_an_offset(mut cursor: Cursor, qd_count: usize) -> Result<usize> {
        for _ in 0..qd_count {
            DomainNameReader::skip(&mut cursor)?;
            cursor.skip(4)?; // qtype(2) + qclass(2)
        }

        Ok(cursor.pos())
    }
}
