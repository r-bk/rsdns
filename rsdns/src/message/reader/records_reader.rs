use crate::{
    bytes::{Cursor, Reader},
    message::{
        reader::{RecordHeader, SectionTracker},
        Header,
    },
    ProtocolError, ProtocolResult,
};

/// Low-level records reader.
///
/// [RecordsReader] provides, at the expense of API ergonomics, the most customizable and
/// efficient way of reading the resource records from a message.
/// For a simple records iterator see [Records](crate::message::reader::Records).
///
/// # Examples
///
/// ```rust
/// use rsdns::{
///     message::reader::MessageReader,
///     Result
/// };
///
/// fn print_headers(msg: &[u8]) -> Result<()> {
///     let mut msg_reader = MessageReader::new(msg)?;
///     let mut rr_reader = msg_reader.records_reader();
///
///     while let Some(header) = rr_reader.peek_header()? {
///         println!(
///             "section: {}, name: {}, rtype: {}, rclass: {}, ttl: {}",
///             header.section(), header.name(), header.rtype(), header.rclass(), header.ttl()
///         );
///         rr_reader.skip_record(&header)?;
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct RecordsReader<'a> {
    cursor: Cursor<'a>,
    section_tracker: SectionTracker,
    err: bool,
}

impl<'a> RecordsReader<'a> {
    pub(crate) fn new(cursor: Cursor<'a>, header: &Header) -> RecordsReader<'a> {
        RecordsReader {
            cursor,
            section_tracker: SectionTracker::new(header),
            err: false,
        }
    }

    /// Peeks the current record header.
    ///
    /// In order to advance to the next record,
    /// the header must be passed to one of the following methods:
    ///
    /// * [RecordsReader::skip_record]
    ///
    /// This method doesn't allocate.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(header))` - if a header was read successfully
    /// * `Ok(None)` - if there are no more records to read, or a previous call resulted with error
    /// * `Err(e)` - on error
    pub fn peek_header(&mut self) -> ProtocolResult<Option<RecordHeader>> {
        if self.err {
            return Ok(None);
        }

        let section = match self.section_tracker.next_section() {
            Some(s) => s,
            None => return Ok(None),
        };

        let mut cursor = self.cursor.clone();

        self.err = true;

        let res = Ok(Some(RecordHeader {
            pos: self.cursor.pos(),
            section,
            name: cursor.read()?,
            name_end_pos: cursor.pos(),
            rtype: cursor.u16_be()?.into(),
            rclass: cursor.u16_be()?.into(),
            ttl: cursor.u32_be()?,
            rdlen: cursor.u16_be()?,
        }));

        self.err = false;

        res
    }

    /// Skips the current record and advances to the next one.
    ///
    /// # Arguments
    ///
    /// * `header` - a record header returned from [RecordsReader::peek_header] method
    ///
    /// # Errors
    ///
    /// * [`HeaderPositionMismatch`] - the header's position doesn't match the reader's position.
    /// This may happen if the same header was used more than once to read or skip the record data.
    ///
    /// [`HeaderPositionMismatch`]: ProtocolError::HeaderPositionMismatch
    pub fn skip_record(&mut self, header: &RecordHeader) -> ProtocolResult<()> {
        if header.pos != self.cursor.pos() {
            return Err(ProtocolError::HeaderPositionMismatch);
        }
        self.cursor
            .set_pos(header.name_end_pos + 10 + header.rdlen as usize);
        Ok(())
    }
}
