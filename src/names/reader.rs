use crate::{
    bytes::{Cursor, Reader},
    message::reader::{read_domain_name, skip_domain_name},
    names::{InlineName, Name},
    Result,
};

pub struct DomainNameReader;

impl DomainNameReader {
    #[inline]
    pub fn read(cursor: &mut Cursor<'_>) -> Result<InlineName> {
        read_domain_name(cursor)
    }

    #[inline]
    pub fn read_string(cursor: &mut Cursor<'_>) -> Result<Name> {
        read_domain_name(cursor)
    }

    #[inline]
    pub fn skip(cursor: &mut Cursor<'_>) -> Result<()> {
        skip_domain_name(cursor)
    }
}

impl Reader<InlineName> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> Result<InlineName> {
        DomainNameReader::read(self)
    }
}

impl Reader<Name> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> Result<Name> {
        DomainNameReader::read_string(self)
    }
}

impl Cursor<'_> {
    pub fn skip_domain_name(&mut self) -> Result<usize> {
        let start = self.pos();
        DomainNameReader::skip(self)?;
        Ok(self.pos() - start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants::DOMAIN_NAME_MAX_POINTERS, Error};

    #[test]
    fn test_basic_flow() {
        let packet = b"\x03sub\x07example\x03com\x00";
        let dn = DomainNameReader::read(&mut Cursor::new(&packet[..])).unwrap();

        assert_eq!(dn.as_str(), "sub.example.com.");
    }

    #[test]
    fn test_basic_flow_string() {
        let packet = b"\x03sub\x07example\x03com\x00";
        let dn = DomainNameReader::read_string(&mut Cursor::new(&packet[..])).unwrap();

        assert_eq!(dn.as_str(), "sub.example.com.");
    }

    #[test]
    fn test_root_domain_name() {
        let packet = b"\x00";

        let dn = DomainNameReader::read(&mut Cursor::new(&packet[..])).unwrap();

        assert_eq!(dn.as_str(), ".");
    }

    #[test]
    fn test_root_domain_name_string() {
        let packet = b"\x00";

        let dn = DomainNameReader::read_string(&mut Cursor::new(&packet[..])).unwrap();

        assert_eq!(dn.as_str(), ".");
    }

    #[test]
    fn test_basic_flow_with_pointers() {
        let packet = b"\x03com\x00\x07example\xC0\x00\x03sub\xC0\x05";

        let dn = DomainNameReader::read(&mut Cursor::with_pos(&packet[..], 15)).unwrap();

        assert_eq!(dn.as_str(), "sub.example.com.");
    }

    #[test]
    fn test_invalid_pointer() {
        let packet = b"\x03com\x00\x07example\xC0\x13\x03sub\xC0\x05";

        assert!(matches!(
            DomainNameReader::read(&mut Cursor::with_pos(&packet[..], 5)),
            Err(Error::DomainNameBadPointer { pointer: p, max_offset: o }) if p == 0x13 && o == 15
        ));
    }

    #[test]
    fn test_pointer_loop() {
        let packet = b"\x03com\x00\x07example\xC0\x0F\x03sub\xC0\x05";

        assert!(matches!(
            DomainNameReader::read(&mut Cursor::with_pos(&packet[..], 15)),
            Err(Error::DomainNameTooMuchPointers)
        ));
    }

    #[test]
    fn test_invalid_label_type() {
        let packet = b"\x03com\x00\x07example\xC0\x0F\x03sub\xA0\x05";

        assert!(matches!(
            DomainNameReader::read(&mut Cursor::with_pos(&packet[..], 15)),
            Err(Error::DomainNameBadLabelType(l)) if l == 0xA0
        ));
    }

    #[test]
    fn test_too_much_pointers() {
        let mut packet: Vec<u8> = b"\x07example\x03com\x00".iter().cloned().collect();
        let start = packet.len();

        for i in 0..DOMAIN_NAME_MAX_POINTERS {
            let offset = if i == 0 { 0 } else { start + 2 * (i - 1) };

            packet.push(0xC0);
            packet.push(offset as u8);
        }

        {
            let dn =
                DomainNameReader::read(&mut Cursor::with_pos(packet.as_ref(), packet.len() - 2))
                    .unwrap();
            assert_eq!(dn.as_str(), "example.com.");
        }

        {
            packet.push(0xC0);
            packet.push((start + 2 * (DOMAIN_NAME_MAX_POINTERS - 1)) as u8);

            assert!(matches!(
                DomainNameReader::read(&mut Cursor::with_pos(packet.as_ref(), packet.len() - 2)),
                Err(Error::DomainNameTooMuchPointers)
            ));
        }
    }

    #[test]
    fn test_cursor_read() {
        let packet = b"\x03sub\x07example\x03com\x00";
        let mut cursor = Cursor::new(&packet[..]);
        let dn: InlineName = cursor.read().unwrap();

        assert_eq!(dn.as_str(), "sub.example.com.");
    }
}
