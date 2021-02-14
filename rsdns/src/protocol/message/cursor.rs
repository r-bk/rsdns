use crate::{protocol::message::BytesReader, Result, RsDnsError};
use std::mem::size_of;

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
}

#[allow(dead_code)]
impl<'a> Cursor<'a> {
    #[inline]
    pub const fn new(buf: &[u8]) -> Cursor {
        Cursor { buf, pos: 0 }
    }

    #[inline]
    pub const fn with_pos(buf: &[u8], pos: usize) -> Cursor {
        Cursor { buf, pos }
    }

    #[inline]
    pub const fn clone_with_pos(&self, pos: usize) -> Cursor {
        Cursor { buf: self.buf, pos }
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos
    }

    pub fn advance(&mut self, distance: usize) -> Result<()> {
        if self.len() >= distance {
            self.pos += distance;
            Ok(())
        } else {
            Err(RsDnsError::EndOfBuffer)
        }
    }

    pub fn len(&self) -> usize {
        let capacity = self.capacity();
        if self.pos < capacity {
            capacity - self.pos
        } else {
            0
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_bytes(&self) -> &[u8] {
        let pos = self.pos.min(self.buf.len());
        unsafe { self.buf.get_unchecked(pos..) }
    }

    pub fn u16_be(&mut self) -> Result<u16> {
        let v = BytesReader::u16_be(self.as_bytes())?;
        self.pos += size_of::<u16>();
        Ok(v)
    }

    pub unsafe fn u16_be_unchecked(&mut self) -> u16 {
        let v = BytesReader::u16_be_unchecked(self.buf.get_unchecked(self.pos..));
        self.pos += size_of::<u16>();
        v
    }

    pub fn u8(&mut self) -> Result<u8> {
        if !self.is_empty() {
            let v = unsafe { *self.buf.get_unchecked(self.pos) };
            self.pos += 1;
            Ok(v)
        } else {
            Err(RsDnsError::EndOfBuffer)
        }
    }

    pub fn slice(&mut self, size: usize) -> Result<&[u8]> {
        if self.len() >= size {
            let pos = self.pos;
            self.pos += size;
            Ok(unsafe { self.buf.get_unchecked(pos..pos + size) })
        } else {
            Err(RsDnsError::EndOfBuffer)
        }
    }
}
