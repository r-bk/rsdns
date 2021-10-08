use crate::{
    bytes::{Cursor, Reader},
    message::Question,
    Error, Result,
};

/// An iterator over the questions section of a message.
///
/// Reading questions doesn't involve memory allocation.
///
/// # Returns
///
/// - `Some(Ok(`[`Question`]`))` - if a question was read successfully
/// - `Some(Err(_))` - on error
/// - `None` - if there is nothing left to read, or a previous call resulted in error
///
/// # Examples
///
/// ```rust
/// # use rsdns::{
/// #     message::reader::MessageIterator,
/// #     Result,
/// # };
/// #
/// # fn print_questions(buf: &[u8]) -> Result<()> {
/// let mut mi = MessageIterator::new(buf)?;
///
/// for result in mi.questions() {
///     let question = result?;
///     println!("{} {} {}", question.qname, question.qtype, question.qclass);
/// }
/// #
/// #   Ok(())
/// # }
/// ```
pub struct Questions<'a> {
    cursor: Cursor<'a>,
    err: bool,
    qd_count: u16,
    qd_read: u16,
}

impl<'a> Questions<'a> {
    pub(crate) fn new(cursor: Cursor<'a>, qd_count: u16) -> Self {
        Self {
            cursor,
            err: false,
            qd_count,
            qd_read: 0,
        }
    }

    fn read(&mut self) -> Option<Result<Question>> {
        if self.err || self.qd_read == self.qd_count {
            return None;
        }

        let res: Result<Question> = self.cursor.read();
        if res.is_ok() {
            self.qd_read += 1;
        } else {
            self.err = true;
        }
        Some(res.map_err(Error::from))
    }
}

impl Iterator for Questions<'_> {
    type Item = Result<Question>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}
