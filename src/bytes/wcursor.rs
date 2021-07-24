use crate::{Error, Result};

#[derive(Debug)]
pub struct WCursor<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> WCursor<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn reset_pos(&mut self) -> usize {
        let mut new_pos = 0;
        std::mem::swap(&mut self.pos, &mut new_pos);
        new_pos
    }

    #[inline]
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
    pub fn slice(&mut self, size: usize) -> Result<&mut [u8]> {
        if self.len() >= size {
            Ok(unsafe { self.buf.get_unchecked_mut(self.pos..self.pos + size) })
        } else {
            Err(Error::BufferTooShort(self.pos() + size))
        }
    }

    #[inline]
    pub fn u16_be(&mut self, val: u16) -> Result<()> {
        w_be!(self, u16, val)
    }

    #[inline]
    pub unsafe fn u16_be_unchecked(&mut self, val: u16) {
        wu_be!(self, u16, val)
    }

    #[inline]
    pub fn u8(&mut self, val: u8) -> Result<()> {
        unsafe { *self.slice(1)?.get_unchecked_mut(0) = val };
        self.pos += 1;
        Ok(())
    }

    #[inline]
    pub unsafe fn u8_unchecked(&mut self, val: u8) {
        *self.buf.get_unchecked_mut(self.pos) = val;
        self.pos += 1;
    }

    #[inline]
    pub unsafe fn bytes_unchecked(&mut self, buf: &[u8]) {
        self.buf
            .get_unchecked_mut(self.pos..self.pos + buf.len())
            .copy_from_slice(buf);
        self.pos += buf.len();
    }
}
