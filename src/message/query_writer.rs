use crate::{
    bytes::{WCursor, Writer},
    constants::{Class, Type},
    message::{Flags, Header},
    records::Opt,
    Result,
};

pub struct QueryWriter<'a> {
    wcursor: WCursor<'a>,
    id: u16,
}

impl<'a> QueryWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        QueryWriter {
            wcursor: WCursor::new(buf),
            id: rand::random::<u16>(),
        }
    }

    #[inline]
    pub fn message_id(&self) -> u16 {
        self.id
    }

    pub fn write(
        &mut self,
        qname: &str,
        qtype: Type,
        qclass: Class,
        recursion_desired: bool,
        opt: Option<Opt>,
    ) -> Result<usize> {
        let header = Header {
            id: self.id,
            flags: *Flags::new().set_recursion_desired(recursion_desired),
            qd_count: 1,
            ar_count: u16::from(opt.is_some()),
            ..Default::default()
        };

        self.wcursor.u16_be(0)?;
        self.wcursor.write(&header)?;
        self.wcursor.write_domain_name(qname)?;
        self.wcursor.u16_be(qtype as u16)?;
        self.wcursor.u16_be(qclass as u16)?;

        if let Some(opt) = opt {
            self.wcursor.write_opt(&opt)?;
        }

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
        let mut qw = QueryWriter::new(&mut query[..]);

        let size = qw
            .write("host.example.com", Type::Cname, Class::In, true, None)
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
        assert_eq!(header.ar_count, 0);
        assert_eq!(dn.as_str(), "host.example.com.");
        assert_eq!(qt, Type::Cname);
        assert_eq!(qc, Class::In);
    }

    #[test]
    fn test_opt() {
        let mut query = [0u8; 512];
        let mut qw = QueryWriter::new(&mut query[..]);

        let payload_size = 4096;
        let ttl = 0x0;
        let opt = Opt::from_msg(payload_size, ttl);

        let size = qw
            .write("host.example.com", Type::Cname, Class::In, false, Some(opt))
            .unwrap();
        assert_eq!(size, 34 + 11 + 2);

        let msg_id = qw.message_id();
        drop(qw);

        let mut c = Cursor::new(&query[..size]);

        let size = c.u16_be().unwrap();
        let header: Header = c.read().unwrap();
        let dn: InlineName = c.read().unwrap();
        let qt = Type::try_from(TypeValue::from(c.u16_be().unwrap())).unwrap();
        let qc = Class::try_from(ClassValue::from(c.u16_be().unwrap())).unwrap();

        let opt_dn: InlineName = c.read().unwrap();
        let opt_rtype = TypeValue(c.u16_be().unwrap());
        let opt_rclass = c.u16_be().unwrap();
        let opt_ttl = c.u32_be().unwrap();
        let opt_rdlen = c.u16_be().unwrap();
        let opt = Opt::from_msg(opt_rclass, opt_ttl);

        assert_eq!(size, 34 + 11);
        assert_eq!(header.flags.recursion_desired(), false);
        assert_eq!(header.id, msg_id);
        assert_eq!(header.qd_count, 1);
        assert_eq!(header.ar_count, 1);
        assert_eq!(dn.as_str(), "host.example.com.");
        assert_eq!(qt, Type::Cname);
        assert_eq!(qc, Class::In);

        assert_eq!(opt_dn.as_str(), ".");
        assert_eq!(opt_rtype, Type::Opt);
        assert_eq!(opt_rclass, payload_size);
        assert_eq!(opt.version(), 0);
        assert_eq!(opt.rcode_extension(), 0);
        assert_eq!(opt.dnssec_ok(), false);
        assert_eq!(opt_rdlen, 0);
    }
}
