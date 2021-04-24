use crate::{Error, Result};

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
}

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
            Err(Error::EndOfBuffer)
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

    pub fn u16_be(&mut self) -> Result<u16> {
        r_be!(self, u16)
    }

    pub unsafe fn u16_be_unchecked(&mut self) -> u16 {
        ru_be!(self, u16)
    }

    pub fn u8(&mut self) -> Result<u8> {
        if !self.is_empty() {
            let v = unsafe { *self.buf.get_unchecked(self.pos) };
            self.pos += 1;
            Ok(v)
        } else {
            Err(Error::EndOfBuffer)
        }
    }

    pub fn slice(&mut self, size: usize) -> Result<&[u8]> {
        if self.len() >= size {
            let pos = self.pos;
            self.pos += size;
            Ok(unsafe { self.buf.get_unchecked(pos..pos + size) })
        } else {
            Err(Error::EndOfBuffer)
        }
    }
}
