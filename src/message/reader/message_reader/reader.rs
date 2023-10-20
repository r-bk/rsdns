use crate::{
    bytes::{Cursor, Reader},
    constants::{Type, HEADER_LENGTH},
    message::{
        reader::{
            NameRef, QuestionRef, RecordHeader, RecordHeaderRef, RecordMarker, RecordOffset,
            SectionTracker,
        },
        ClassValue, Header, Question, RecordsSection, TypeValue,
    },
    names::DName,
    records::{data::RData, Opt},
    Error, Result,
};

#[derive(Debug)]
/// A fast and flexible message reader.
///
/// `MessageReader` provides a flexible API for traversing a DNS message.
/// It doesn't implement the `Iterator` trait, and, consequently, it doesn't support the Rust `for`
/// loop. However, because it is not bound to a single type of item, it provides more flexibility
/// than an iterator-based reader. For instance, it allows reading the record data into dedicated
/// Rust types, without enclosing them in an artificial enum (which is usually done for returning
/// multiple types of elements from an `Iterator`).
///
/// The API of `MessageReader` is roughly divided into three parts:
///   1. A method to read the message header
///   2. Methods to read the Questions section
///   3. Methods to read the resource records (Answers, Authority and Additional sections)
///
/// DNS message sections do not have a constant size. Thus, in order to position a `MessageReader`
/// at a specific element of a message, all previous elements must be read first.
///
///
/// # Message Header
///
/// The message header is read using the [`header`] method.
///
/// The header contains information about the layout of the sections that follow.
/// It must be read immediately after creation of a `MessageReader`. Without this
/// information `MessageReader` is unaware of the amount of elements in each of the message
/// sections, and behaves as if there are zero elements in all sections.
///
/// [`header`]: MessageReader::header
///
///
/// # Questions
///
/// The methods to read the Questions section are:
///
/// 1. [`has_questions`]
/// 2. [`questions_count`]
/// 3. [`question`] and [`question_ref`]
/// 4. [`the_question`] and [`the_question_ref`]
/// 5. [`skip_questions`]
///
/// The Questions section is the first section immediately following the header. The DNS protocol
/// allows more than one question to be encoded in a message. However, this is not used in practice,
/// and usually every message contains a single question only.
///
/// DNS question is represented in *rsdns* by the types [`Question`] and [`QuestionRef`].
///
/// The functions [`question`] and [`question_ref`] read and return the next question, and
/// are intended to be used as follows:
///
/// ```rust
/// # use rsdns::{message::reader::MessageReader, Result};
/// # fn print_questions(msg: &[u8]) -> Result<()> {
/// let mut mr = MessageReader::new(msg)?;
/// mr.header()?;
/// while mr.has_questions() {
///     let q = mr.question()?; // or mr.question_ref()
///     // use q ...
/// # drop(q);
/// }
/// #
/// # Ok(())
/// # }
/// ```
///
/// The functions [`the_question`] and [`the_question_ref`] are useful when exactly one
/// question is expected in the message. They return [`Error::BadQuestionsCount`] if the number
/// of questions is anything other than `1`.
///
/// ```rust
/// # use rsdns::{message::reader::MessageReader, Result};
/// # fn print_questions(msg: &[u8]) -> Result<()> {
/// let mut mr = MessageReader::new(msg)?;
/// mr.header()?;
/// let q = mr.the_question()?; // or mr.the_question_ref()
/// // use q ...
/// # drop(q);
/// # Ok(())
/// # }
/// ```
///
/// Finally, if questions are not of any interest, the whole section may be skipped using the
/// [`skip_questions`] method:
///
/// ```rust
/// # use rsdns::{message::reader::MessageReader, Result};
/// # fn print_questions(msg: &[u8]) -> Result<()> {
/// let mut mr = MessageReader::new(msg)?;
/// mr.header()?;
/// mr.skip_questions()?;
/// // the reader is positioned to decode the resource records...
/// # Ok(())
/// # }
/// ```
///
/// [`has_questions`]: MessageReader::has_questions
/// [`questions_count`]: MessageReader::questions_count
/// [`question`]: MessageReader::question
/// [`question_ref`]: MessageReader::question_ref
/// [`the_question`]: MessageReader::the_question
/// [`the_question_ref`]: MessageReader::the_question_ref
/// [`skip_questions`]: MessageReader::skip_questions
///
///
/// # Resource Records
///
/// The methods to read the resource record sections (Answers, Authority and Additional) are:
///
/// 1. [`has_records`] and [`has_records_in`]
/// 2. [`records_count`] and [`records_count_in`]
/// 3. [`record_marker`] `(G1)`
/// 4. [`record_header`] and [`record_header_ref`] `(G1)`
/// 5. [`record_data`] and [`record_data_bytes`] `(G2)`
/// 6. [`skip_record_data`] `(G2)`
/// 7. [`opt_record`]
///
/// Reading a resource record is a two-step process. Firstly, the record header must be read using
/// any method in group `G1`. Secondly, (immediately after) the record data must be read using any
/// method in group `G2`. Every call to a method in `G1` must be followed by a call to a method in
/// `G2`. Every call to a method in `G2` must be preceded by a call to a method from `G1`.
///
/// [`has_records`]: MessageReader::has_records
/// [`has_records_in`]: MessageReader::has_records_in
/// [`records_count`]: MessageReader::records_count
/// [`records_count_in`]: MessageReader::records_count_in
/// [`record_marker`]: MessageReader::record_marker
/// [`record_header`]: MessageReader::record_header
/// [`record_header_ref`]: MessageReader::record_header_ref
/// [`record_data`]: MessageReader::record_data
/// [`record_data_bytes`]: MessageReader::record_data_bytes
/// [`skip_record_data`]: MessageReader::skip_record_data
/// [`opt_record`]: MessageReader::opt_record
///
/// ## Marker, Header and HeaderRef
///
/// The types [`RecordMarker`], [`RecordHeader`] and [`RecordHeaderRef`] are used to parse a record
/// header. The marker holds all record header fields except the domain name. This information
/// is required to correctly parse both the domain name and the record data that follows the header.
/// The `RecordHeader` and `RecordHeaderRef` types add the domain name to `RecordMarker`. The
/// difference between them is similar to the difference between [`Question`] and [`QuestionRef`].
/// `RecordHeader` owns the domain name bytes by using a type implementing the [`DName`] trait.
/// `RecordHeaderRef` doesn't own the domain name bytes, and points back to the encoded domain name
/// in the message buffer. This allows efficient comparison of the domain name to a domain name of
/// another record or the question.
///
/// Ideally these three types would be implemented in a single type `RecordHeader` with a generic
/// type parameter for the domain name. However, as of now, Rust doesn't allow having both
/// [`NameRef`] and [`DName`] hidden behind the same trait.
///
///
/// # EDNS OPT pseudo-record
///
/// The EDNS `OPT` pseudo-record is handled slightly differently than other record types.
/// It has a dedicated method [`opt_record`] which completes reading the record data, and returns
/// the [`Opt`] struct which holds `OPT` values from both record header and record data parts.
///
///
/// # Reader Exhaustion and Error State
///
/// When all records have been read and the reader is exhausted, an attempt to read another record
/// fails with [`Error::ReaderDone`].
///
/// If an error occurs during parsing of any element, the reader enters an error state and behaves
/// as if it is exhausted.
///
///
/// # Random Access
///
/// Random access methods are:
///
/// 1. [`record_data_at`]
/// 2. [`record_data_bytes_at`]
/// 3. [`name_ref_at`]
///
/// These methods allow random access to record data, assuming the record markers are first
/// traversed and stored for later processing.
///
/// Note that these methods are immutable, they do not change the internal buffer pointer of
/// the reader.
///
/// [`record_data_at`]: MessageReader::record_data_at
/// [`record_data_bytes_at`]: MessageReader::record_data_bytes_at
/// [`name_ref_at`]: MessageReader::name_ref_at
///
///
/// # Seeking
///
/// Seeking is possible using the [`seek`] method.
///
/// When `MessageReader` traverses a message and reaches the first record of any of the records
/// sections, it internally stores the offset of the record. This allows seeking to the beginning
/// of an already known section in constant time.
///
/// Additionally, a seek to an unknown section position is possible immediately after the header is
/// read. In this case the reader will decode all the elements until it is positioned at the first
/// record of the requested section.
///
/// [`seek`]: MessageReader::seek
///
///
/// # Examples
///
/// ```rust
/// use rsdns::{
///     constants::{Type, RCode},
///     message::{reader::MessageReader, RecordsSection},
///     names::Name,
///     records::data::{A, Aaaa},
///     Error, Result,
/// };
///
/// fn print_answer_addresses(msg: &[u8]) -> Result<()> {
///     let mut mr = MessageReader::new(msg)?;
///     let header = mr.header()?;
///
///     let rcode = header.flags.response_code();
///     if rcode != RCode::NoError {
///         return Err(Error::BadResponseCode(rcode));
///     }
///
///     if header.flags.truncated() {
///         return Err(Error::MessageTruncated);
///     }
///
///     mr.seek(RecordsSection::Answer)?;
///
///     while mr.has_records_in(RecordsSection::Answer) {
///         let rh = mr.record_header::<Name>()?;
///
///         if rh.rtype() == Type::A {
///             let rdata = mr.record_data::<A>(rh.marker())?;
///             println!(
///                 "Name: {}; Class: {}; TTL: {}; ipv4: {}",
///                 rh.name(), rh.rclass(), rh.ttl(), rdata.address
///             );
///         } else if rh.rtype() == Type::Aaaa {
///             let rdata = mr.record_data::<Aaaa>(rh.marker())?;
///             println!(
///                 "Name: {}; Class: {}; TTL: {}; ipv6: {}",
///                 rh.name(), rh.rclass(), rh.ttl(), rdata.address
///             );
///         } else {
///             // every record must be read fully: header + data
///             mr.skip_record_data(rh.marker())?;
///         }
///     }
///
///     Ok(())
/// }
/// #
/// # // A cnn.com
/// # #[rustfmt::skip]
/// # const M0: [u8; 89] = [
/// #    0xe4, 0xe9, 0x81, 0x80, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, // |............| 0
/// #    0x03, 0x63, 0x6e, 0x6e, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, // |.cnn.com....| 12
/// #    0x01, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x22, 0x00, // |..........".| 24
/// #    0x04, 0x97, 0x65, 0x01, 0x43, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, // |..e.C.......| 36
/// #    0x00, 0x00, 0x22, 0x00, 0x04, 0x97, 0x65, 0xc1, 0x43, 0xc0, 0x0c, 0x00, // |.."...e.C...| 48
/// #    0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x22, 0x00, 0x04, 0x97, 0x65, 0x41, // |......"...eA| 60
/// #    0x43, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x22, 0x00, // |C.........".| 72
/// #    0x04, 0x97, 0x65, 0x81, 0x43, /*                                     */ // |..e.C| 84
/// # ];
/// # print_answer_addresses(&M0[..]).unwrap();
/// ```
pub struct MessageReader<'a> {
    cursor: Cursor<'a>,
    section_tracker: SectionTracker,
    done: bool,
}

impl<'s, 'a: 's> MessageReader<'a> {
    /// Creates a `MessageReader` for a given message.
    ///
    /// This method only minimally initializes the `MessageReader's` state. The message header must
    /// be read immediately after creation of the reader in order to finalize its initialization
    /// and properly read the rest of the message.
    ///
    /// # Returns
    ///
    /// - [`Error::MessageTooLong`] - if message size exceeds 65535 bytes.
    #[inline]
    pub fn new(msg: &'a [u8]) -> Result<MessageReader<'a>> {
        if msg.len() > u16::MAX as usize {
            return Err(Error::MessageTooLong(msg.len()));
        }
        Ok(MessageReader {
            cursor: Cursor::new(msg),
            section_tracker: Default::default(),
            done: false,
        })
    }

    /// Reads the message header.
    ///
    /// This is the first method that must be called after creation of a `MessageReader`.
    /// It reads the message header and initializes internal counters to properly
    /// read the rest of the message.
    #[inline]
    pub fn header(&mut self) -> Result<Header> {
        let res = self.header_impl();
        if res.is_err() {
            self.done = true;
        }
        res
    }

    #[inline(always)]
    fn header_impl(&mut self) -> Result<Header> {
        let header = self.cursor.read()?;
        self.section_tracker.set(&header);
        Ok(header)
    }

    /// Positions the message reader to the first record of a specific section.
    ///
    /// Seeking is possible only in one of the two scenarios:
    ///
    /// 1. The message reader was just created and the header was read.
    ///    In this case the reader will skip all message data until it positions itself at the
    ///    first record of the requested section.
    /// 2. The message was read up to (and including) the last record of the first non-empty section
    ///    preceding the requested one, or any record beyond that.
    ///
    /// As a message is traversed, the reader remembers offsets of its sections. So, when a
    /// message was entirely traversed, it is possible to seek to any section.
    ///
    /// Note that if the requested section is empty, the first record to be read after seek may
    /// belong to a consecutive section, or no records may be left at all.
    pub fn seek(&mut self, section: RecordsSection) -> Result<()> {
        if self.done {
            return Err(Error::ReaderDone);
        }

        if let Some(offset) = self.section_tracker.section_offset(section) {
            self.cursor.set_pos(offset);
            self.section_tracker.seek(section);
            return Ok(());
        }

        if self.cursor.pos() != HEADER_LENGTH {
            return Err(Error::RecordsSectionOffsetUnknown(section));
        }

        let res = self.seek_impl(section);
        if res.is_err() {
            self.done = true;
        }
        res
    }

    #[inline(always)]
    fn seek_impl(&mut self, section: RecordsSection) -> Result<()> {
        self.skip_questions_impl()?;
        match section {
            RecordsSection::Answer => Ok(()),
            RecordsSection::Authority => self.skip_section_impl(RecordsSection::Answer),
            RecordsSection::Additional => {
                self.skip_section_impl(RecordsSection::Answer)?;
                self.skip_section_impl(RecordsSection::Authority)
            }
        }
    }

    #[inline(always)]
    fn skip_section_impl(&mut self, section: RecordsSection) -> Result<()> {
        while self.section_tracker.records_left_in(section) > 0 {
            let marker = self.marker_impl()?;
            self.skip_record_data_impl(&marker)?;
        }
        Ok(())
    }

    /// Checks if there are more questions to read.
    ///
    /// This is a convenience method. It is equivalent to [`questions_count()`]` > 0`.
    ///
    /// [`questions_count()`]: Self::questions_count
    #[inline]
    pub fn has_questions(&self) -> bool {
        self.questions_count() > 0
    }

    /// Returns the number of unread questions.
    ///
    /// Returns `0` if the reader is in error state.
    #[inline]
    pub fn questions_count(&self) -> usize {
        if !self.done {
            self.section_tracker.questions_left()
        } else {
            0
        }
    }

    /// Reads the next question.
    #[inline]
    pub fn question(&mut self) -> Result<Question> {
        question!(self, question_check_no_questions)
    }

    /// Reads the next question as [`QuestionRef`].
    #[inline]
    pub fn question_ref(&'s mut self) -> Result<QuestionRef<'a>> {
        question!(self, question_check_no_questions)
    }

    /// Reads the first and only question.
    ///
    /// This method is equivalent to [`question`], except that it returns
    /// [`Error::BadQuestionsCount`] if the number of questions is not `1`.
    ///
    /// [`question`]: MessageReader::question
    #[inline]
    pub fn the_question(&mut self) -> Result<Question> {
        question!(self, question_check_single_question)
    }

    /// Reads the first and only question as [`QuestionRef`].
    ///
    /// This method is equivalent to [`question_ref`], except that it returns
    /// [`Error::BadQuestionsCount`] if the number of questions is not `1`.
    ///
    /// [`question_ref`]: MessageReader::question_ref
    #[inline]
    pub fn the_question_ref(&'s mut self) -> Result<QuestionRef<'a>> {
        question!(self, question_check_single_question)
    }

    /// Skips the questions section.
    ///
    /// This is a convenience method to advance the reader to the end of the questions section.
    ///
    /// Note that this method may be called only immediately after the header is read.
    pub fn skip_questions(&mut self) -> Result<()> {
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = self.skip_questions_impl();
        if res.is_err() {
            self.done = true;
        }
        res
    }

    #[inline(always)]
    fn skip_questions_impl(&mut self) -> Result<()> {
        while self.section_tracker.questions_left() > 0 {
            self.cursor.skip_question()?;
            self.section_tracker.question_read(self.cursor.pos());
        }
        Ok(())
    }

    /// Checks if there are more records to read.
    ///
    /// This is a convenience method. It is equivalent to [`records_count()`]` > 0`.
    ///
    /// [`records_count()`]: Self::records_count
    #[inline]
    pub fn has_records(&self) -> bool {
        self.records_count() > 0
    }

    /// Checks if there are more records to read in a specific section.
    ///
    /// This is a convenience method. It is equivalent to [`records_count_in()`]` > 0`.
    ///
    /// [`records_count_in()`]: Self::records_count_in
    #[inline]
    pub fn has_records_in(&self, section: RecordsSection) -> bool {
        self.records_count_in(section) > 0
    }

    /// Returns the number of unread records.
    ///
    /// Returns `0` if the reader is in error state.
    #[inline]
    pub fn records_count(&self) -> usize {
        if !self.done {
            self.section_tracker.records_left()
        } else {
            0
        }
    }

    /// Returns the number of unread records in a specific section.
    ///
    /// Returns `0` if the reader is in error state.
    #[inline]
    pub fn records_count_in(&self, section: RecordsSection) -> usize {
        if !self.done {
            self.section_tracker.records_left_in(section)
        } else {
            0
        }
    }

    /// Returns the marker of the current resource record.
    #[inline]
    pub fn record_marker(&mut self) -> Result<RecordMarker> {
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = self.marker_impl();
        if res.is_err() {
            self.done = true;
        }
        res
    }

    #[inline(always)]
    fn marker_impl(&mut self) -> Result<RecordMarker> {
        let pos = self.cursor.pos();
        let section = self.calc_section()?;
        self.cursor.skip_domain_name()?;
        self.raw_marker_impl(pos, section)
    }

    #[inline(always)]
    fn raw_marker_impl(&mut self, pos: usize, section: RecordsSection) -> Result<RecordMarker> {
        let offset = RecordOffset {
            offset: pos,
            type_offset: self.cursor.pos(),
        };

        let rtype = TypeValue::from(self.cursor.u16_be()?);
        let rclass = ClassValue::from(self.cursor.u16_be()?);
        let ttl = self.cursor.u32_be()?;
        let rdlen = self.cursor.u16_be()?;

        Ok(RecordMarker {
            offset,
            rtype,
            rclass,
            ttl,
            rdlen,
            section,
        })
    }

    /// Reads the header of the current resource record as [`RecordHeaderRef`].
    #[inline]
    pub fn record_header_ref(&'s mut self) -> Result<RecordHeaderRef<'a>> {
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = self.record_header_ref_impl();
        if res.is_err() {
            self.done = true;
        }
        res
    }

    #[inline(always)]
    fn record_header_ref_impl(&'s mut self) -> Result<RecordHeaderRef<'a>> {
        let pos = self.cursor.pos();
        let section = self.calc_section()?;
        let name_ref = NameRef::new(self.cursor.clone());
        self.cursor.skip_domain_name()?;
        let marker = self.raw_marker_impl(pos, section)?;
        Ok(RecordHeaderRef { name_ref, marker })
    }

    /// Reads the header of the current resource record.
    ///
    /// This method is generic over the domain name type `N` used for the name of the record.
    /// This allows parsing the header without memory allocations, if appropriate type is used.
    #[inline]
    pub fn record_header<N: DName>(&mut self) -> Result<RecordHeader<N>> {
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = self.record_header_impl::<N>();
        if res.is_err() {
            self.done = true;
        }
        res
    }

    #[inline(always)]
    fn record_header_impl<N: DName>(&mut self) -> Result<RecordHeader<N>> {
        let pos = self.cursor.pos();
        let section = self.calc_section()?;
        let name = N::from_cursor(&mut self.cursor)?;
        let marker = self.raw_marker_impl(pos, section)?;
        Ok(RecordHeader { name, marker })
    }

    /// Skips the current record data and advances the reader to the next record.
    ///
    /// # Panics
    ///
    /// This method uses debug assertions to verify that `marker` matches the reader's buffer
    /// pointer.
    #[inline]
    pub fn skip_record_data(&mut self, marker: &RecordMarker) -> Result<()> {
        debug_assert!(self.cursor.pos() == marker.rdata_pos());
        if self.done {
            return Err(Error::ReaderDone);
        }
        self.skip_record_data_impl(marker)
    }

    #[inline(always)]
    fn skip_record_data_impl(&mut self, marker: &RecordMarker) -> Result<()> {
        let res = self.cursor.skip(marker.rdlen as usize);
        if res.is_ok() {
            self.section_tracker
                .section_read(marker.section, self.cursor.pos());
        } else {
            self.done = true;
        }
        res
    }

    /// Returns the current record data as a byte slice and advances the reader to the next record.
    ///
    /// This method allows reading data of unknown record types, as defined in [RFC 3597 section 5].
    ///
    /// # Panics
    ///
    /// This method uses debug assertions to verify that `marker` matches the reader's buffer
    /// pointer.
    ///
    /// [RFC 3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    #[inline]
    pub fn record_data_bytes(&'s mut self, marker: &RecordMarker) -> Result<&'a [u8]> {
        debug_assert!(self.cursor.pos() == marker.rdata_pos());
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = self.cursor.slice(marker.rdlen as usize);
        if res.is_ok() {
            self.section_tracker
                .section_read(marker.section, self.cursor.pos());
        } else {
            self.done = true;
        }
        res
    }

    /// Deserializes the current record data and advances the reader to the next record.
    ///
    /// This method is generic over the record data type, and allows deserialization of all
    /// data types supported by *rsdns*. See [`RData`] for the full list of record data types
    /// implementing the trait.
    ///
    /// # Panics
    ///
    /// This method uses debug assertions to verify that `marker` matches the reader's buffer
    /// pointer.
    #[inline]
    pub fn record_data<D: RData>(&mut self, marker: &RecordMarker) -> Result<D> {
        debug_assert!(self.cursor.pos() == marker.rdata_pos());
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = D::from_cursor(&mut self.cursor, marker.rdlen as usize);
        if res.is_ok() {
            self.section_tracker
                .section_read(marker.section, self.cursor.pos());
        } else {
            self.done = true;
        }
        res
    }

    /// Reads the `OPT` pseudo-record and advances the reader to the next record.
    ///
    /// # Panics
    ///
    /// This method uses debug assertions to verify that:
    ///
    /// - the `marker` matches the reader's buffer pointer
    /// - the record type of the marker is [`Opt`](Type::Opt)
    #[inline]
    pub fn opt_record(&mut self, marker: &RecordMarker) -> Result<Opt> {
        if self.done {
            return Err(Error::ReaderDone);
        }
        debug_assert!(self.cursor.pos() == marker.rdata_pos());
        debug_assert!(marker.rtype == Type::Opt);
        let res = self.opt_record_impl(marker);
        if res.is_ok() {
            self.section_tracker
                .section_read(marker.section, self.cursor.pos());
        } else {
            self.done = true;
        }
        res
    }

    #[inline(always)]
    fn opt_record_impl(&mut self, marker: &RecordMarker) -> Result<Opt> {
        self.cursor.skip(marker.rdlen as usize)?;
        Ok(Opt::from_msg(marker.rclass.0, marker.ttl))
    }

    /// Reads the data of a record at specified marker and returns it as a byte slice.
    ///
    /// This method allows random access to the encoded records of a DNS message.
    /// It is intended to be used in cases when record headers are read in one loop,
    /// while record data is later (possibly selectively) read in another loop. If data is read
    /// together with the header, use [`MessageReader::record_data_bytes`] instead, which is more
    /// efficient.
    ///
    /// Note that this method is immutable and doesn't change the reader's buffer pointer.
    /// Nor it is affected by an error state of the reader.
    #[inline]
    pub fn record_data_bytes_at(&'s self, marker: &RecordMarker) -> Result<&'a [u8]> {
        let mut cursor = self.cursor.clone_with_pos(marker.rdata_pos());
        cursor.slice(marker.rdlen as usize)
    }

    /// Reads and deserializes the data of a record at specified marker.
    ///
    /// This method allows random access to the encoded records of a DNS message.
    /// It is intended to be used in cases when record headers are read in one loop,
    /// while record data is later (possibly selectively) read in another loop. If data is read
    /// together with the header, use [`MessageReader::record_data`] instead, which is more
    /// efficient.
    ///
    /// Note that this method is immutable and doesn't change the reader's buffer pointer.
    /// Nor it is affected by an error state of the reader.
    #[inline]
    pub fn record_data_at<D: RData>(&self, marker: &RecordMarker) -> Result<D> {
        let mut cursor = self.cursor.clone_with_pos(marker.rdata_pos());
        D::from_cursor(&mut cursor, marker.rdlen as usize)
    }

    /// Returns the data of a record at specified marker as [`NameRef`].
    ///
    /// This method is handy with records that have a single domain name in the
    /// data section, e.g. `CNAME`, `NS`, `PTR` etc.
    #[inline]
    pub fn name_ref_at(&'s self, marker: &RecordMarker) -> NameRef<'a> {
        NameRef::new(self.cursor.clone_with_pos(marker.rdata_pos()))
    }

    #[inline(always)]
    fn calc_section(&mut self) -> Result<RecordsSection> {
        self.section_tracker
            .next_section(self.cursor.pos())
            .ok_or(Error::ReaderDone)
    }
}
