use crate::{
    bytes::{WCursor, Writer},
    constants::{Class, Type},
    message::{Flags, Header},
    Result,
};

pub struct QueryWriter<'a> {
    wcursor: WCursor<'a>,
    id: u16,
    recursion_desired: bool,
}

impl<'a> QueryWriter<'a> {
    pub fn new(buf: &'a mut [u8], recursion_desired: bool) -> Self {
        QueryWriter {
            wcursor: WCursor::new(buf),
            id: rand::random::<u16>(),
            recursion_desired,
        }
    }

    #[inline]
    pub fn message_id(&self) -> u16 {
        self.id
    }

    pub fn write(&mut self, qname: &str, qtype: Type, qclass: Class) -> Result<usize> {
        let header = Header {
            id: self.id,
            flags: *Flags::new().set_recursion_desired(self.recursion_desired),
            qd_count: 1,
            ..Default::default()
        };

        self.wcursor.u16_be(0)?;
        self.wcursor.write(&header)?;
        self.wcursor.write_domain_name(qname)?;
        self.wcursor.u16_be(qtype as u16)?;
        self.wcursor.u16_be(qclass as u16)?;
        let pos = self.wcursor.reset_pos();
        self.wcursor.u16_be((pos - 2) as u16)?;
        Ok(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bytes::{Cursor, Reader},
        message::{ClassValue, TypeValue},
        names::InlineName,
    };
    use std::convert::TryFrom;

    #[test]
    fn test_good_flow() {
        let mut query = [0u8; 512];
        let mut qw = QueryWriter::new(&mut query[..], true);

        let size = qw
            .write("host.example.com", Type::Cname, Class::In)
            .unwrap();
        assert_eq!(size, 34 + 2);

        let msg_id = qw.message_id();
        drop(qw);

        let mut c = Cursor::new(&query[..size]);

        let size = c.u16_be().unwrap();
        let header: Header = c.read().unwrap();
        let dn: InlineName = c.read().unwrap();
        let qt = Type::try_from(TypeValue::from(c.u16_be().unwrap())).unwrap();
        let qc = Class::try_from(ClassValue::from(c.u16_be().unwrap())).unwrap();

        assert_eq!(size, 34);
        assert!(header.flags.recursion_desired());
        assert_eq!(header.id, msg_id);
        assert_eq!(header.qd_count, 1);
        assert_eq!(dn.as_str(), "host.example.com.");
        assert_eq!(qt, Type::Cname);
        assert_eq!(qc, Class::In);
    }
}
