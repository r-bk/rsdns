use crate::{
    bytes::{Cursor, Reader, WCursor, Writer},
    constants::HEADER_LENGTH,
    message::Flags,
    ProtocolError, ProtocolResult,
};

/// Message header.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Header {
    /// An identifier assigned by the program that generates any kind of query.
    /// This identifier is copied to the corresponding reply and can be used by the requester to
    /// match up replies to outstanding requests.
    pub id: u16,
    /// Message flags.
    pub flags: Flags,
    /// Number of entries in the question section.
    pub qd_count: u16,
    /// Number of resource records in the answer section.
    pub an_count: u16,
    /// Number of name server resource records in the authority records section.
    pub ns_count: u16,
    /// Number of resource records in the additional records section.
    pub ar_count: u16,
}

impl Writer<Header> for WCursor<'_> {
    fn write(&mut self, h: &Header) -> ProtocolResult<usize> {
        if self.len() >= HEADER_LENGTH {
            unsafe {
                self.u16_be_unchecked(h.id);
                self.u16_be_unchecked(h.flags.into());
                self.u16_be_unchecked(h.qd_count);
                self.u16_be_unchecked(h.an_count);
                self.u16_be_unchecked(h.ns_count);
                self.u16_be_unchecked(h.ar_count);
            }
            Ok(HEADER_LENGTH)
        } else {
            Err(ProtocolError::EndOfBuffer)
        }
    }
}

impl Reader<Header> for Cursor<'_> {
    fn read(&mut self) -> ProtocolResult<Header> {
        if self.len() >= HEADER_LENGTH {
            unsafe {
                Ok(Header {
                    id: self.u16_be_unchecked(),
                    flags: Flags::from(self.u16_be_unchecked()),
                    qd_count: self.u16_be_unchecked(),
                    an_count: self.u16_be_unchecked(),
                    ns_count: self.u16_be_unchecked(),
                    ar_count: self.u16_be_unchecked(),
                })
            }
        } else {
            Err(ProtocolError::EndOfBuffer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{OpCode, RCode};
    use rand::seq::IteratorRandom;

    #[test]
    fn test_serialization() {
        let mut rng = rand::thread_rng();

        let flags = *Flags::new()
            .set_message_type(rand::random::<bool>().into())
            .set_authoritative_answer(rand::random())
            .set_recursion_available(rand::random())
            .set_recursion_desired(rand::random())
            .set_truncated(rand::random())
            .set_opcode(*OpCode::VALUES.iter().choose(&mut rng).unwrap())
            .set_response_code(*RCode::VALUES.iter().choose(&mut rng).unwrap());

        let header = Header {
            id: rand::random::<u16>(),
            flags,
            qd_count: rand::random::<u16>(),
            an_count: rand::random::<u16>(),
            ns_count: rand::random::<u16>(),
            ar_count: rand::random::<u16>(),
        };

        let mut buf = [0u8; HEADER_LENGTH];

        {
            let mut wcursor = WCursor::new(&mut buf[..]);
            wcursor.write(&header).unwrap();
        }

        let mut cursor = Cursor::new(&buf[..]);

        let another = cursor.read().unwrap();

        assert_eq!(header, another);
    }

    #[test]
    fn test_serializaton_end_of_buffer() {
        let mut empty_arr = [0u8; 0];
        let mut small_arr = [0u8; HEADER_LENGTH - 1];

        let res: ProtocolResult<Header> = Cursor::new(&empty_arr[..]).read();
        assert!(matches!(res, Err(ProtocolError::EndOfBuffer)));

        let res: ProtocolResult<Header> = Cursor::new(&small_arr[..]).read();
        assert!(matches!(res, Err(ProtocolError::EndOfBuffer)));

        let header = Header::default();

        assert!(matches!(
            WCursor::new(&mut empty_arr[..]).write(&header),
            Err(ProtocolError::EndOfBuffer)
        ));

        assert!(matches!(
            WCursor::new(&mut small_arr[..]).write(&header),
            Err(ProtocolError::EndOfBuffer)
        ));
    }
}
