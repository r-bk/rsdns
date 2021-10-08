use crate::{
    bytes::Cursor,
    constants::RecordsSection,
    message::{
        reader::{
            NameRef, RecordHeader, RecordHeaderRef, RecordMarker, RecordOffset, SectionTracker,
        },
        ClassValue, Header, TypeValue,
    },
    names::DName,
    records::data::RData,
    Error, Result,
};

#[derive(Debug)]
/// A flexible reader of resource records.
///
/// `RecordsReader` provides a flexible API for traversing the resource records of a DNS message.
/// It doesn't implement the `Iterator` trait, so it doesn't support the Rust `for` loop.
/// However, as it is not bound to a single type of item, it provides great flexibility in
/// parsing the resource records.
///
/// The API of `RecordsReader` is roughly divided into two parts:
///
/// 1. Methods to read a record header: [`RecordsReader::marker`], [`RecordsReader::header`] and
///    [`RecordsReader::header_ref`].
/// 2. Methods to read record data: [`RecordsReader::data`], [`RecordsReader::data_bytes`] and
///    [`RecordsReader::skip_data`].
///
/// Every call to a method in group (1) must be followed by a call to a method from group (2),
/// before the next record can be read. Every call to a method in group (2) must be preceded by
/// a call to a method from group (1).
///
/// The methods [`RecordsReader::has_records`] and [`RecordsReader::count`] exist
/// to check if there are more records to read.
///
/// When all records have been read and the reader is exhausted, an attempt to read another record
/// fails with [`Error::ReaderDone`]. If an error occurs during parsing of a record header or
/// data, the reader enters an error state and behaves as if it is exhausted.
///
/// # `Marker`, `Header` and `HeaderRef`
///
/// The types [`RecordMarker`], [`RecordHeader`] and [`RecordHeaderRef`] are used to parse a record
/// header. The marker holds all record header fields except for the domain name. This information
/// is required to correctly parse both the domain name and the record data that follows the header.
/// The `RecordHeader` and `RecordHeaderRef` types add the domain name to the `RecordMarker`.
/// The difference is the type used for the domain name. `RecordHeader` owns the domain name bytes
/// by using a type implementing the `DName` trait. `RecordHeaderRef` doesn't own the domain name
/// bytes, and points back to the encoded domain name in the message buffer. This allows efficient
/// comparison of the domain name to a domain name of another record or the question.
///
/// Ideally these three types would be implemented in a single type `RecordHeader` with a generic
/// type parameter for the domain name. However, as of now, Rust doesn't allow having both `NameRef`
/// and `DName` hidden behind the same trait.
///
/// # Random Access
///
/// `RecordsReader` has additional methods [`RecordsReader::data_at`],
/// [`RecordsReader::data_bytes_at`] and [`RecordsReader::name_ref_at`].
/// These methods allow random access to record data, assuming the record markers are first
/// traversed and stored for later processing.
///
/// Note that these methods are immutable, they do not change the internal buffer pointer of
/// the reader.
///
/// # Examples
///
/// ```rust
/// use rsdns::{
///     constants::{RecordsSection, Type, RCode},
///     message::reader::MessageIterator,
///     names::Name,
///     records::data::{A, Aaaa},
///     Error, Result,
/// };
///
/// fn print_answer_addresses(msg: &[u8]) -> Result<()> {
///     let mut mi = MessageIterator::new(msg)?;
///
///     let rcode = mi.header().flags.response_code();
///     if rcode != RCode::NoError {
///         return Err(Error::BadResponseCode(rcode));
///     }
///
///     if mi.header().flags.truncated() {
///         return Err(Error::MessageTruncated);
///     }
///
///     let mut rr = mi.records_reader_for(RecordsSection::Answer)?;
///
///     while rr.has_records() {
///         let header = rr.header::<Name>()?;
///
///         if header.rtype() == Type::A {
///             let rdata = rr.data::<A>(header.marker())?;
///             println!(
///                 "Name: {}; Class: {}; TTL: {}; ipv4: {}",
///                 header.name(), header.rclass(), header.ttl(), rdata.address
///             );
///         } else if header.rtype() == Type::Aaaa {
///             let rdata = rr.data::<Aaaa>(header.marker())?;
///             println!(
///                 "Name: {}; Class: {}; TTL: {}; ipv6: {}",
///                 header.name(), header.rclass(), header.ttl(), rdata.address
///             );
///         } else {
///             // every record must be read fully: header + data
///             rr.skip_data(header.marker())?;
///         }
///     }
///
///     Ok(())
/// }
/// ```
pub struct RecordsReader<'a> {
    cursor: Cursor<'a>,
    section_tracker: SectionTracker,
    done: bool,
}

impl<'s, 'a: 's> RecordsReader<'a> {
    #[inline(always)]
    pub(crate) fn new<'c>(cursor: Cursor<'c>, header: &Header) -> RecordsReader<'c> {
        RecordsReader {
            cursor,
            section_tracker: SectionTracker::new(header),
            done: false,
        }
    }

    #[inline(always)]
    pub(crate) fn with_section<'c>(
        cursor: Cursor<'c>,
        header: &Header,
        section: RecordsSection,
    ) -> RecordsReader<'c> {
        RecordsReader {
            cursor,
            section_tracker: SectionTracker::with_section(header, section),
            done: false,
        }
    }

    /// Checks if there are more records to read.
    ///
    /// This is a convenience method. It is equivalent to [`count()`]` > 0`.
    ///
    /// [`count()`]: Self::count
    #[inline]
    pub fn has_records(&self) -> bool {
        self.count() > 0
    }

    /// Returns the number of unread records.
    ///
    /// Returns `0` if the reader is in error state.
    #[inline]
    pub fn count(&self) -> usize {
        if !self.done {
            self.section_tracker.records_left()
        } else {
            0
        }
    }

    /// Returns the marker of the current resource record.
    #[inline]
    pub fn marker(&mut self) -> Result<RecordMarker> {
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
        let section = self.calc_section()?;
        let pos = self.cursor.pos();
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
    pub fn header_ref(&'s mut self) -> Result<RecordHeaderRef<'a>> {
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = self.header_ref_impl();
        if res.is_err() {
            self.done = true;
        }
        res
    }

    #[inline(always)]
    fn header_ref_impl(&'s mut self) -> Result<RecordHeaderRef<'a>> {
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
    pub fn header<N: DName>(&mut self) -> Result<RecordHeader<N>> {
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = self.header_impl::<N>();
        if res.is_err() {
            self.done = true;
        }
        res
    }

    #[inline(always)]
    fn header_impl<N: DName>(&mut self) -> Result<RecordHeader<N>> {
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
    /// This method uses debug assertions to verify that the `marker` matches the reader's buffer
    /// pointer.
    #[inline]
    pub fn skip_data(&mut self, marker: &RecordMarker) -> Result<()> {
        debug_assert!(self.cursor.pos() == marker.rdata_pos());
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = self.cursor.skip(marker.rdlen as usize);
        if res.is_ok() {
            self.section_tracker.section_read(marker.section)?;
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
    /// This method uses debug assertions to verify that the `marker` matches the reader's buffer
    /// pointer.
    ///
    /// [RFC 3597 section 5]: https://www.rfc-editor.org/rfc/rfc3597.html#section-5
    #[inline]
    pub fn data_bytes(&'s mut self, marker: &RecordMarker) -> Result<&'a [u8]> {
        debug_assert!(self.cursor.pos() == marker.rdata_pos());
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = self.cursor.slice(marker.rdlen as usize);
        if res.is_ok() {
            self.section_tracker.section_read(marker.section)?;
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
    /// This method uses debug assertions to verify that the `marker` matches the reader's buffer
    /// pointer.
    #[inline]
    pub fn data<D: RData>(&mut self, marker: &RecordMarker) -> Result<D> {
        debug_assert!(self.cursor.pos() == marker.rdata_pos());
        if self.done {
            return Err(Error::ReaderDone);
        }
        let res = D::from_cursor(&mut self.cursor, marker.rdlen as usize);
        if res.is_ok() {
            self.section_tracker.section_read(marker.section)?;
        } else {
            self.done = true;
        }
        res
    }

    /// Reads the data of a record at specified marker and returns it as a byte slice.
    ///
    /// This method allows random access to the encoded records of a DNS message.
    /// It is intended to be used in cases when record headers are read in one loop,
    /// while record data is later (possibly selectively) read in another loop. If data is read
    /// together with the header, use [`RecordsReader::data_bytes`] instead, which is more
    /// efficient.
    ///
    /// Note that this method is immutable and doesn't change the reader's buffer pointer.
    /// Nor it is affected by an error state of the reader.
    #[inline]
    pub fn data_bytes_at(&'s self, marker: &RecordMarker) -> Result<&'a [u8]> {
        let mut cursor = self.cursor.clone_with_pos(marker.rdata_pos());
        cursor.slice(marker.rdlen as usize)
    }

    /// Reads and deserializes the data of a record at specified marker.
    ///
    /// This method allows random access to the encoded records of a DNS message.
    /// It is intended to be used in cases when record headers are read in one loop,
    /// while record data is later (possibly selectively) read in another loop. If data is read
    /// together with the header, use [`RecordsReader::data`] instead, which is more
    /// efficient.
    ///
    /// Note that this method is immutable and doesn't change the reader's buffer pointer.
    /// Nor it is affected by an error state of the reader.
    #[inline]
    pub fn data_at<D: RData>(&self, marker: &RecordMarker) -> Result<D> {
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
        self.section_tracker.next_section().ok_or(Error::ReaderDone)
    }
}
