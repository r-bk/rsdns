use crate::{
    bytes::{Cursor, Reader},
    constants::{RecordsSection, HEADER_LENGTH},
    message::{
        reader::{Questions, Records},
        Header, Question,
    },
    Error, Result,
};

/// An iterator-based message reader.
///
/// `MessageIterator` is a utility for parsing DNS messages. It allows parsing all of the DNS
/// message components, the header, the questions section and resource records sections.
///
/// `MessageIterator` implements an `Iterator`-based approach for parsing a message. The methods
/// [`MessageIterator::questions`] and [`MessageIterator::records`] return types which implement
/// the `Iterator` trait. This makes the API convenient to use with the Rust's `for` loop. However,
/// convenience comes with a price of slightly slower performance. `Iterator` requires definition
/// of a single item type. Thus, to support different resource record types in a single item type,
/// the [`RecordData`] enum is defined. Consequently, accessing the record data requires enum
/// destructuring.
///
/// [`RecordData`]: crate::records::data::RecordData
///
/// # Examples
///
/// ```rust
/// use rsdns::{
///     constants::RecordsSection,
///     message::reader::MessageIterator,
///     records::data::RecordData,
/// };
///
/// fn print_answers(buf: &[u8]) -> rsdns::Result<()> {
///     let mi = MessageIterator::new(buf)?;
///
///     let header = mi.header();
///
///     println!("ID: {}", header.id);
///     println!("Type: {}", header.flags.message_type());
///     println!("Questions: {} Answers: {}", header.qd_count, header.an_count);
///
///     let q = mi.question()?;
///     println!("Question: {} {} {}", q.qname, q.qtype, q.qclass);
///
///     for result in mi.records() {
///         let (section, record) = result?;
///
///         if section != RecordsSection::Answer {
///             // Answer is the first section; skip the rest
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
/// ```
#[derive(Debug)]
pub struct MessageIterator<'a> {
    buf: &'a [u8],
    header: Header,
    offsets: [usize; 3],
}

impl<'s, 'a: 's> MessageIterator<'a> {
    /// Creates a reader for a message contained in `buf`.
    #[inline]
    pub fn new(buf: &[u8]) -> Result<MessageIterator> {
        let mut cursor = Cursor::new(buf);
        let header: Header = cursor.read()?;
        let mut mi = MessageIterator {
            buf,
            header,
            offsets: [0, 0, 0],
        };
        // pre-calculate the Answers offset for backward compatibility
        mi.section_offset(RecordsSection::Answer)?;
        Ok(mi)
    }

    /// Returns the parsed header.
    #[inline]
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// Returns the first question in the questions section.
    ///
    /// Usually a DNS message contains a single question.
    #[inline]
    pub fn question(&self) -> Result<Question> {
        let mut questions = self.questions();
        if let Some(res) = questions.next() {
            return res;
        }
        Err(Error::BadQuestionsCount(0))
    }

    /// Returns an iterator over the questions section of the message.
    #[inline]
    pub fn questions(&self) -> Questions {
        Questions::new(
            Cursor::with_pos(self.buf, HEADER_LENGTH),
            self.header.qd_count,
        )
    }

    /// Returns an iterator over the resource record sections of the message.
    #[inline]
    pub fn records(&self) -> Records {
        Records::new(
            Cursor::with_pos(self.buf, self.offsets[RecordsSection::Answer as usize]),
            &self.header,
        )
    }

    fn section_offset(&mut self, section: RecordsSection) -> Result<usize> {
        use RecordsSection::*;

        let existing_value = self.offsets[section as usize];
        if existing_value != 0 {
            return Ok(existing_value);
        }

        match section {
            Answer => {
                let mut c = Cursor::with_pos(self.buf, HEADER_LENGTH);
                for _ in 0..self.header.qd_count {
                    c.skip_question()?;
                }
                let offset = c.pos();
                self.offsets[Answer as usize] = offset;
                Ok(offset)
            }
            Authority => {
                let answer_offset = self.section_offset(Answer)?;
                let mut c = Cursor::with_pos(self.buf, answer_offset);
                for _ in 0..self.header.an_count {
                    c.skip_rr()?;
                }
                let offset = c.pos();
                self.offsets[Authority as usize] = offset;
                Ok(offset)
            }
            Additional => {
                let authority_offset = self.section_offset(Authority)?;
                let mut c = Cursor::with_pos(self.buf, authority_offset);
                for _ in 0..self.header.ns_count {
                    c.skip_rr()?;
                }
                let offset = c.pos();
                self.offsets[Additional as usize] = offset;
                Ok(offset)
            }
        }
    }
}
