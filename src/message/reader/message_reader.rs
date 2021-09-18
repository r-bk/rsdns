use crate::{
    bytes::{Cursor, Reader},
    constants::{RecordsSection, HEADER_LENGTH},
    message::{
        reader::{QuestionRef, Questions, Records, RecordsReader},
        Header, Question,
    },
    Error, Result,
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
///     let q = mr.question()?;
///     println!("Question: {} {} {}", q.qname, q.qtype, q.qclass);
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
pub struct MessageReader<'a> {
    buf: &'a [u8],
    header: Header,
    offsets: [usize; 3],
}

impl<'a> MessageReader<'a> {
    /// Creates a reader for a message contained in `buf`.
    pub fn new(buf: &'a [u8]) -> Result<Self> {
        let mut cursor = Cursor::new(buf);
        let header: Header = cursor.read()?;
        let offsets = Self::find_section_offsets(cursor, &header)?;
        Ok(MessageReader {
            buf,
            header,
            offsets,
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
        Err(Error::MessageWithoutQuestion)
    }

    /// Returns the first question in the questions section as `QuestionRef`.
    ///
    /// Usually a DNS message contains a single question.
    pub fn question_ref(&self) -> Result<QuestionRef<'a>> {
        if self.header.qd_count == 0 {
            return Err(Error::MessageWithoutQuestion);
        }
        let mut cursor = Cursor::with_pos(self.buf, HEADER_LENGTH);
        cursor.read()
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
        Records::new(
            Cursor::with_pos(self.buf, self.section_offset(RecordsSection::Answer)),
            &self.header,
        )
    }

    /// Returns a records reader for all records sections.
    #[inline]
    pub fn records_reader(&self) -> RecordsReader {
        RecordsReader::new(
            Cursor::with_pos(self.buf, self.section_offset(RecordsSection::Answer)),
            &self.header,
        )
    }

    /// Returns a records reader for a specific records section.
    #[inline]
    pub fn records_reader_for(&self, section: RecordsSection) -> RecordsReader {
        RecordsReader::with_section(
            Cursor::with_pos(self.buf, self.section_offset(section)),
            &self.header,
            section,
        )
    }

    fn find_section_offsets(mut cursor: Cursor, header: &Header) -> Result<[usize; 3]> {
        use RecordsSection::*;

        let mut ans = [0, 0, 0];

        // skip Question section
        for _ in 0..header.qd_count {
            cursor.skip_question()?;
        }
        ans[Answer as usize] = cursor.pos();

        // skip Answer section
        for _ in 0..header.an_count {
            cursor.skip_rr()?;
        }
        ans[Authority as usize] = cursor.pos();

        // skip Authority section
        for _ in 0..header.ns_count {
            cursor.skip_rr()?;
        }
        ans[Additional as usize] = cursor.pos();

        Ok(ans)
    }

    fn section_offset(&self, section: RecordsSection) -> usize {
        self.offsets[section as usize]
    }
}
