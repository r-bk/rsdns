use crate::{
    protocol::{
        bytes::{Cursor, WCursor},
        constants::HEADER_LENGTH,
        Flags,
    },
    Error, Result,
};

/// DNS message header.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[derive(Debug, Default, Eq, PartialEq)]
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

impl Header {
    pub(crate) fn read(cursor: &mut Cursor) -> Result<Header> {
        if cursor.len() >= HEADER_LENGTH {
            unsafe {
                Ok(Header {
                    id: cursor.u16_be_unchecked(),
                    flags: Flags::from(cursor.u16_be_unchecked()),
                    qd_count: cursor.u16_be_unchecked(),
                    an_count: cursor.u16_be_unchecked(),
                    ns_count: cursor.u16_be_unchecked(),
                    ar_count: cursor.u16_be_unchecked(),
                })
            }
        } else {
            Err(Error::EndOfBuffer)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn write(&self, cursor: &mut WCursor) -> Result<()> {
        if cursor.len() >= HEADER_LENGTH {
            unsafe {
                cursor.u16_be_unchecked(self.id);
                cursor.u16_be_unchecked(self.flags.as_u16());
                cursor.u16_be_unchecked(self.qd_count);
                cursor.u16_be_unchecked(self.an_count);
                cursor.u16_be_unchecked(self.ns_count);
                cursor.u16_be_unchecked(self.ar_count);
                Ok(())
            }
        } else {
            Err(Error::EndOfBuffer)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{OpCode, RCode};
    use rand::seq::IteratorRandom;
    use strum::IntoEnumIterator;

    #[test]
    fn test_serialization() {
        let mut rng = rand::thread_rng();

        let flags = Flags::new()
            .set_qr(rand::random())
            .set_aa(rand::random())
            .set_ra(rand::random())
            .set_rd(rand::random())
            .set_tc(rand::random())
            .set_opcode(OpCode::iter().choose(&mut rng).unwrap())
            .set_rcode(RCode::iter().choose(&mut rng).unwrap());

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
            header.write(&mut wcursor).unwrap();
        }

        let mut cursor = Cursor::new(&buf[..]);

        let another = Header::read(&mut cursor).unwrap();

        assert_eq!(header, another);
    }

    #[test]
    fn test_serializaton_end_of_buffer() {
        let mut empty_arr = [0u8; 0];
        let mut small_arr = [0u8; HEADER_LENGTH - 1];

        assert!(matches!(
            Header::read(&mut Cursor::new(&empty_arr[..])),
            Err(Error::EndOfBuffer)
        ));

        assert!(matches!(
            Header::read(&mut Cursor::new(&small_arr[..])),
            Err(Error::EndOfBuffer)
        ));

        let header = Header::default();

        assert!(matches!(
            header.write(&mut WCursor::new(&mut empty_arr[..])),
            Err(Error::EndOfBuffer)
        ));

        assert!(matches!(
            header.write(&mut WCursor::new(&mut small_arr[..])),
            Err(Error::EndOfBuffer)
        ));
    }
}
