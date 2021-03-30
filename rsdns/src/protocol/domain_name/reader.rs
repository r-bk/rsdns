use crate::{
    protocol::{
        bytes::{Cursor, Reader},
        DomainName,
    },
    Error, Result,
};

const POINTER_MASK: u8 = 0b1100_0000;
const LENGTH_MASK: u8 = 0b0011_1111;
const MAX_POINTERS: usize = 32;

type OffsetsArray = arrayvec::ArrayVec<u16, MAX_POINTERS>;

pub struct DomainNameReader<'a> {
    cursor: Cursor<'a>,
    max_pos: usize,
    seen_offsets: OffsetsArray,
}

impl<'a> DomainNameReader<'a> {
    pub fn read(cursor: &mut Cursor<'a>) -> Result<DomainName> {
        let mut dn = DomainName::new();
        let mut dnr = Self::new(cursor.clone());
        dnr.read_impl(&mut dn)?;
        cursor.set_pos(dnr.max_pos);
        Ok(dn)
    }

    pub fn skip(cursor: &mut Cursor<'a>) -> Result<()> {
        let mut dnr = Self::new(cursor.clone());
        dnr.skip_impl()?;
        cursor.set_pos(dnr.max_pos);
        Ok(())
    }
}

impl<'a> DomainNameReader<'a> {
    fn new(cursor: Cursor<'a>) -> Self {
        DomainNameReader {
            cursor,
            max_pos: 0,
            seen_offsets: OffsetsArray::default(),
        }
    }

    fn skip_impl(&mut self) -> Result<()> {
        loop {
            let length = self.cursor.u8()?;
            if length == 0 {
                break;
            } else if Self::is_length(length) {
                self.cursor.advance(length as usize)?;
            } else if Self::is_pointer(length) {
                let o2 = self.cursor.u8()?;
                let offset = Self::pointer_to_offset(length, o2);

                if self.max_pos == 0 {
                    self.max_pos = self.cursor.pos();
                }
                if offset as usize > self.max_pos {
                    return Err(Error::DomainNameBadPointer);
                }
                self.remember_offset(offset)?;
                self.cursor.set_pos(offset as usize);
            } else {
                return Err(Error::DomainNameBadLabelType);
            }
        }

        if self.max_pos == 0 {
            self.max_pos = self.cursor.pos();
        }
        Ok(())
    }

    fn read_impl(&mut self, dn: &mut DomainName) -> Result<()> {
        loop {
            let length = self.cursor.u8()?;
            if length == 0 {
                break;
            } else if Self::is_length(length) {
                let label = self.cursor.slice(length as usize)?;
                dn.push_label_bytes(label)?;
            } else if Self::is_pointer(length) {
                let o2 = self.cursor.u8()?;
                let offset = Self::pointer_to_offset(length, o2);

                if self.max_pos == 0 {
                    self.max_pos = self.cursor.pos();
                }
                if offset as usize > self.max_pos {
                    return Err(Error::DomainNameBadPointer);
                }
                self.remember_offset(offset)?;
                self.cursor.set_pos(offset as usize);
            } else {
                return Err(Error::DomainNameBadLabelType);
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

    fn remember_offset(&mut self, offset: u16) -> Result<()> {
        for o in &self.seen_offsets {
            if *o == offset {
                return Err(Error::DomainNamePointerLoop);
            }
        }
        if self.seen_offsets.is_full() {
            return Err(Error::DomainNameTooMuchPointers);
        }
        unsafe { self.seen_offsets.push_unchecked(offset) };
        Ok(())
    }
}

impl Reader<DomainName> for Cursor<'_> {
    #[inline]
    fn read(&mut self) -> Result<DomainName> {
        DomainNameReader::read(self)
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
    fn test_root_domain_name() {
        let packet = b"\x00";

        let dn = DomainNameReader::read(&mut Cursor::new(&packet[..])).unwrap();

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
            Err(Error::DomainNameBadPointer)
        ));
    }

    #[test]
    fn test_pointer_loop() {
        let packet = b"\x03com\x00\x07example\xC0\x0F\x03sub\xC0\x05";

        assert!(matches!(
            DomainNameReader::read(&mut Cursor::with_pos(&packet[..], 15)),
            Err(Error::DomainNamePointerLoop)
        ));
    }

    #[test]
    fn test_invalid_label_type() {
        let packet = b"\x03com\x00\x07example\xC0\x0F\x03sub\xA0\x05";

        assert!(matches!(
            DomainNameReader::read(&mut Cursor::with_pos(&packet[..], 15)),
            Err(Error::DomainNameBadLabelType)
        ));
    }

    #[test]
    fn test_too_much_pointers() {
        let mut packet: Vec<u8> = b"\x07example\x03com\x00".iter().cloned().collect();
        let start = packet.len();

        for i in 0..MAX_POINTERS {
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
            packet.push((start + 2 * (MAX_POINTERS - 1)) as u8);

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
        let dn: DomainName = cursor.read().unwrap();

        assert_eq!(dn.as_str(), "sub.example.com.");
    }
}
