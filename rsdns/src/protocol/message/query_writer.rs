use crate::{
    protocol::{
        bytes::{WCursor, Writer},
        Flags, Header, QClass, QType,
    },
    Result,
};

pub struct QueryWriter<'a> {
    wcursor: WCursor<'a>,
    id: u16,
    rd: bool,
}

#[allow(dead_code)]
impl<'a> QueryWriter<'a> {
    pub fn new(buf: &'a mut [u8], recursion_desired: bool) -> Self {
        QueryWriter {
            wcursor: WCursor::new(buf),
            id: rand::random::<u16>(),
            rd: recursion_desired,
        }
    }

    #[inline]
    pub fn message_id(&self) -> u16 {
        self.id
    }

    pub fn write(&mut self, qname: &str, qtype: QType, qclass: QClass) -> Result<usize> {
        let header = Header {
            id: self.id,
            flags: Flags::new().set_rd(self.rd),
            qd_count: 1,
            ..Default::default()
        };

        self.wcursor.write(&header)?;
        self.wcursor.write_domain_name(qname)?;
        self.wcursor.u16_be(qtype as u16)?;
        self.wcursor.u16_be(qclass as u16)?;
        Ok(self.wcursor.pos())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{
        bytes::{Cursor, Reader},
        DomainName,
    };
    use std::convert::TryFrom;

    #[test]
    fn test_good_flow() {
        let mut query = [0u8; 512];
        let mut qw = QueryWriter::new(&mut query[..], true);

        let size = qw
            .write("host.example.com", QType::CNAME, QClass::IN)
            .unwrap();
        assert_eq!(size, 34);

        let msg_id = qw.message_id();
        drop(qw);

        let mut c = Cursor::new(&query[..size]);

        let header: Header = c.read().unwrap();
        let dn: DomainName = c.read().unwrap();
        let qt = QType::try_from(c.u16_be().unwrap()).unwrap();
        let qc = QClass::try_from(c.u16_be().unwrap()).unwrap();

        assert!(header.flags.rd());
        assert_eq!(header.id, msg_id);
        assert_eq!(header.qd_count, 1);
        assert_eq!(dn.as_str(), "host.example.com.");
        assert_eq!(qt, QType::CNAME);
        assert_eq!(qc, QClass::IN);
    }
}
