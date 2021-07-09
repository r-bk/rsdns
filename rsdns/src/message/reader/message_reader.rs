use crate::{
    bytes::{Cursor, Reader},
    constants::HEADER_LENGTH,
    errors::{Error, ProtocolError, Result},
    message::{
        reader::{Questions, Records},
        Header, Question,
    },
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
///     constants::RecordsSection,
///     message::reader::MessageReader,
///     records::data::RecordData,
/// };
///
/// fn print_answers(buf: &[u8]) -> rsdns::Result<()> {
///     let mr = MessageReader::new(buf)?;
///
///     let header = mr.header();
///
///     println!("ID: {}", header.id);
///     println!("Type: {}", header.flags.message_type());
///     println!("Questions: {} Answers: {}", header.qd_count, header.an_count);
///
///     for (index, question) in mr.questions().enumerate() {
///         let q = question?;
///         println!("Question {}: {} {} {}", index, q.qname, q.qtype, q.qclass);
///     }
///
///     for result in mr.records() {
///         let (section, record) = result?;
///
///         if section != RecordsSection::Answer {
///             break;
///         }
///
///         match record.rdata {
///             RecordData::Cname(ref rdata) => {
///                 println!(
///                     "Name: {}; Class: {}; TTL: {}; Cname: {}",
///                     record.name, record.rclass, record.ttl, rdata.cname
///                 );
///             }
///             RecordData::A(ref rdata) => {
///                 println!(
///                     "Name: {}; Class: {}; TTL: {}; ipv4: {}",
///                     record.name, record.rclass, record.ttl, rdata.address
///                 );
///             }
///             RecordData::Aaaa(ref rdata) => {
///                 println!(
///                     "Name: {}; Class: {}; TTL: {}; ipv6: {}",
///                     record.name, record.rclass, record.ttl, rdata.address
///                 );
///             }
///             _ => println!("{:?}", record),
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

    /// Returns the first question in the questions section.
    ///
    /// Usually a DNS message contains a single question.
    pub fn question(&self) -> Result<Question> {
        let mut questions = self.questions();
        if let Some(res) = questions.next() {
            return res;
        }
        Err(Error::ProtocolError(ProtocolError::NoQuestion))
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
            cursor.skip_domain_name()?;
            cursor.skip(4)?; // qtype(2) + qclass(2)
        }

        Ok(cursor.pos())
    }
}
