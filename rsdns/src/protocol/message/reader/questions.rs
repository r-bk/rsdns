use crate::{
    protocol::{bytes::Cursor, Question},
    Result,
};

/// A reader of the questions section of a DNS message.
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

    /// Reads the next question.
    ///
    /// Returns:
    ///
    /// - `Ok(Some(Question))` - if a question was read successfully
    /// - `Ok(None)` - if there is nothing left to read, or a previous call resulted in error
    /// - `Err(_)` - on error
    ///
    /// # Examples
    ///
    /// There are several possible ways to read all questions.
    ///
    /// One is to use the `for` loop:
    ///
    /// ```
    /// use rsdns::{
    ///     protocol::{
    ///         message::MessageReader,
    ///         Question
    ///     },
    ///     Result
    /// };
    ///
    /// fn print_questions(buf: &[u8]) -> Result<()> {
    ///     let mut message_reader = MessageReader::new(buf)?;
    ///     let mut questions = message_reader.questions();
    ///
    ///     for question in questions.read()? {
    ///         println!(
    ///             "{} {} {}",
    ///             question.qname.as_str(),
    ///             question.qtype as u16,
    ///             question.qclass as u16
    ///         );
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Or, if more elaborate error handling is required, using `loop`:
    ///
    /// ```
    /// use rsdns::{
    ///     protocol::{
    ///         message::MessageReader,
    ///         Question
    ///     },
    ///     Result
    /// };
    ///
    /// fn print_questions(buf: &[u8]) -> Result<()> {
    ///     let mut message_reader = MessageReader::new(buf)?;
    ///     let mut questions = message_reader.questions();
    ///
    ///     loop {
    ///         match questions.read() {
    ///             Ok(Some(question)) => println!("{:?}", question),
    ///             Ok(None) => break,
    ///             Err(e) => {
    ///                 /* error handling here */
    ///                 return Err(e);
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn read(&mut self) -> Result<Option<Question>> {
        if self.err || self.qd_read == self.qd_count {
            return Ok(None);
        }

        match Question::read(&mut self.cursor) {
            Ok(q) => {
                self.qd_read += 1;
                Ok(Some(q))
            }
            Err(e) => {
                self.err = true;
                Err(e)
            }
        }
    }
}
