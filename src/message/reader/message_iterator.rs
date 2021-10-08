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
/// `MessageIterator` is the main utility for parsing messages. It allows parsing all of the DNS
/// message components, the header, the questions section and resource records sections -
/// answer, authority and additional.
///
/// # Header
///
/// The header of a DNS message is parsed in the constructor [`MessageIterator::new`] and can be
/// obtained with the [`MessageIterator::header`] method. An error during header parsing fails the
/// creation of a message reader.
///
/// # Questions
///
/// DNS message format allows encoding more than one Question in the same message. However,
/// it isn't really possible to ask more than one question in the same request.
/// Hence a message usually contains only a single question. `MessageIterator` returns an iterator
/// over the questions section via the [`MessageIterator::questions`] method.
/// Additionally, a helper method [`MessageIterator::question`] exists which returns just the first
/// (and usually the only) question.
///
/// Note that struct [`Question`] owns the domain name, i.e. domain name
/// is decoded from the DNS message and copied into a sequential buffer. This is not always the
/// most efficient way of comparing the domain name in the question to domain names in the
/// resource records sections. When DNS [message compression] is in use, most of
/// records in the answers section will usually point to the domain name in the question.
/// For this `MessageIterator` has the [`MessageIterator::question_ref`] method that returns the
/// first question as struct [`QuestionRef`]. The difference is that `QuestionRef` doesn't own the
/// domain name bytes, but rather points back to the encoded domain name in the message buffer.
/// This allows efficient comparison of domain names encoded in the **same** DNS message, assuming
/// DNS message compression is in use (which is usually the case with most DNS recursors and proxy
/// servers).
///
/// # Records
///
/// `MessageIterator` provides two ways for parsing the resource records:
/// 1. The [`MessageIterator::records`] method which returns an iterator over all records sections.
/// 2. The [`MessageIterator::records_reader`] method which returns a non-iterator records reader.
///
/// ## The `records` method
///
/// The [`MessageIterator::records`] method returns an iterator [`Records`] to iterate over
/// `Answer`, `Authority` and `Additional` sections of a message.
/// This is the simplest form of traversing the records, as `Records`
/// implements the `Iterator` trait, and, as such, supports Rust's [`for`] loop.
///
/// The downside of `Records` is that, as a Rust iterator, it must have a single item type to
/// yield in the loop. Thus, all supported data types in the [`records::data`] module are
/// grouped into the [`RecordData`] enum, which is used in the item type of the iterator.
/// This creates a need to destructure the enum to obtain the actual resource record data, which
/// may be less aesthetic, especially when many different data types are involved.
///
/// Also, note that `Records` implicitly skips resource records whose type is not yet
/// part of the `RecordData` enum.
///
/// ## The `records_reader` method
///
/// The [`MessageIterator::records_reader`] method returns struct [`RecordsReader`], which is
/// another type that allows traversal of records in a message. It tries to do so without the
/// drawbacks associated with [`Records`] mentioned above.
///
/// `RecordsReader` is not a Rust `Iterator`. It is not bound to a single
/// type of item on every iteration. Hence, record data can be obtained from it directly,
/// without artificial enum enclosing. Moreover, record types still not supported by *rsdns*
/// can be read as byte slices. So, no record is implicitly skipped. This allows an application
/// to handle unknown record types as defined in [RFC 3597 section 5].
///
/// Additionally, `RecordsReader` can be bound to a single message section using the
/// [`MessageIterator::records_reader_for`] method. There is no such alternative with the `Records`
/// iterator.
///
/// [`records::data`]: crate::records::data
/// [`for`]: https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops
/// [`RecordData`]: crate::records::data::RecordData
/// [message compression]: https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.4
/// [RFC 3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
///
/// # Examples
///
/// See [`Records`] for an example of using the iterator approach.
///
/// See [`RecordsReader`] for an example of using the non-iterator approach.
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
        Err(Error::MessageWithoutQuestion)
    }

    /// Returns the first question in the questions section as `QuestionRef`.
    ///
    /// Usually a DNS message contains a single question.
    #[inline]
    pub fn question_ref(&self) -> Result<QuestionRef<'a>> {
        if self.header.qd_count == 0 {
            return Err(Error::MessageWithoutQuestion);
        }
        let mut cursor = Cursor::with_pos(self.buf, HEADER_LENGTH);
        cursor.read()
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

    /// Returns a records reader for all records sections.
    #[inline]
    pub fn records_reader(&self) -> RecordsReader {
        let offset = self.offsets[RecordsSection::Answer as usize];
        RecordsReader::new(Cursor::with_pos(self.buf, offset), &self.header)
    }

    /// Returns a records reader for a specific records section.
    #[inline]
    pub fn records_reader_for(&'s mut self, section: RecordsSection) -> Result<RecordsReader<'a>> {
        let offset = self.section_offset(section)?;
        Ok(RecordsReader::with_section(
            Cursor::with_pos(self.buf, offset),
            &self.header,
            section,
        ))
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
