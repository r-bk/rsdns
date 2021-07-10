use crate::{
    bytes::{Cursor, Reader},
    constants::DOMAIN_NAME_MAX_POINTERS,
    errors::{ProtocolError, ProtocolResult},
    InlineName, Name, NameContract,
};

const POINTER_MASK: u8 = 0b1100_0000;
const LENGTH_MASK: u8 = 0b0011_1111;

type OffsetsArray = arrayvec::ArrayVec<u16, DOMAIN_NAME_MAX_POINTERS>;

pub struct DomainNameReader<'a> {
    cursor: Cursor<'a>,
    max_pos: usize,
    seen_offsets: OffsetsArray,
}

impl<'a> DomainNameReader<'a> {
    pub fn read(cursor: &mut Cursor<'a>) -> ProtocolResult<InlineName> {
        let mut dn = InlineName::new();
        Self::read_internal(cursor, &mut dn)?;
        Ok(dn)
    }

    pub fn read_string(cursor: &mut Cursor<'a>) -> ProtocolResult<Name> {
        let mut dn = Name::new();
        Self::read_internal(cursor, &mut dn)?;
        Ok(dn)
    }

    fn read_internal<T: NameContract>(cursor: &mut Cursor<'a>, dn: &mut T) -> ProtocolResult<()> {
        let mut dnr = Self::new(cursor.clone());
        dnr.read_impl(dn)?;
        cursor.set_pos(dnr.max_pos);
        Ok(())
    }

    pub fn skip(cursor: &mut Cursor<'a>) -> ProtocolResult<()> {
        let mut dnr = Self::new(cursor.clone());
        dnr.skip_impl()?;
        cursor.set_pos(dnr.max_pos);
        Ok(())
    }

    fn new(cursor: Cursor<'a>) -> Self {
        DomainNameReader {
            cursor,
            max_pos: 0,
            seen_offsets: OffsetsArray::default(),
        }
    }

    fn skip_impl(&mut self) -> ProtocolResult<()> {
        loop {
            let length = self.cursor.u8()?;
            if length == 0 {
                break;
            } else if Self::is_length(length) {
                self.cursor.skip(length as usize)?;
            } else if Self::is_pointer(length) {
                let o2 = self.cursor.u8()?;
                let offset = Self::pointer_to_offset(length, o2);

                if self.max_pos == 0 {
                    self.max_pos = self.cursor.pos();
                }
                if offset as usize > self.max_pos {
                    return Err(ProtocolError::DomainNameBadPointer);
                }
                self.remember_offset(offset)?;
                self.cursor.set_pos(offset as usize);
            } else {
                return Err(ProtocolError::DomainNameBadLabelType(length));
            }
        }

        if self.max_pos == 0 {
            self.max_pos = self.cursor.pos();
        }
        Ok(())
    }

    fn read_impl<T: NameContract>(&mut self, dn: &mut T) -> ProtocolResult<()> {
        loop {
            let length = self.cursor.u8()?;
            if length == 0 {
                break;
            } else if Self::is_length(length) {
                let label = self.cursor.slice(length as usize)?;
                dn.append_label_bytes(label)?;
            } else if Self::is_pointer(length) {
                let o2 = self.cursor.u8()?;
                let offset = Self::pointer_to_offset(length, o2);

                if self.max_pos == 0 {
                    self.max_pos = self.cursor.pos();
                }
                if offset as usize > self.max_pos {
                    return Err(ProtocolError::DomainNameBadPointer);
                }
                self.remember_offset(offset)?;
                self.cursor.set_pos(offset as usize);
            } else {
                return Err(ProtocolError::DomainNameBadLabelType(length));
            }
        }

        if dn.is_empty() {
            dn.set_root();
        }
        if self.max_pos == 0 {
            self.max_pos = self.cursor.pos();
        }
        Ok(())
    }

    #[inline]
    const fn is_pointer(b: u8) -> bool {
        (b & POINTER_MASK) == POINTER_MASK
    }

    #[inline]
    const fn is_length(b: u8) -> bool {
        (b & LENGTH_MASK) == b
    }

    #[inline]
    const fn pointer_to_offset(o1: u8, o2: u8) -> u16 {
        (((o1 & LENGTH_MASK) as u16) << 8) | o2 as u16
    }

    fn remember_offset(&mut self, offset: u16) -> ProtocolResult<()> {
        for o in &self.seen_offsets {
            if *o == offset {
                return Err(ProtocolError::DomainNamePointerLoop {
                    src: self.cursor.pos() - 2, // the offset of the label's first byte
                    dst: offset as usize,
                });
            }
        }
        if self.seen_offsets.is_full() {
            return Err(ProtocolError::DomainNameTooMuchPointers);
        }
        unsafe { self.seen_offsets.push_unchecked(offset) };
        Ok(())
    }
}

impl Reader<InlineName> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> ProtocolResult<InlineName> {
        DomainNameReader::read(self)
    }
}

impl Reader<Name> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> ProtocolResult<Name> {
        DomainNameReader::read_string(self)
    }
}

impl Cursor<'_> {
    pub fn skip_domain_name(&mut self) -> ProtocolResult<usize> {
        let start = self.pos();
        DomainNameReader::skip(self)?;
        Ok(self.pos() - start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            Err(ProtocolError::DomainNameBadPointer)
        ));
    }

    #[test]
    fn test_pointer_loop() {
        let packet = b"\x03com\x00\x07example\xC0\x0F\x03sub\xC0\x05";

        assert!(matches!(
            DomainNameReader::read(&mut Cursor::with_pos(&packet[..], 15)),
            Err(ProtocolError::DomainNamePointerLoop { src, dst }) if src == 19 && dst == 5
        ));
    }

    #[test]
    fn test_invalid_label_type() {
        let packet = b"\x03com\x00\x07example\xC0\x0F\x03sub\xA0\x05";

        assert!(matches!(
            DomainNameReader::read(&mut Cursor::with_pos(&packet[..], 15)),
            Err(ProtocolError::DomainNameBadLabelType(l)) if l == 0xA0
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
                Err(ProtocolError::DomainNameTooMuchPointers)
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
