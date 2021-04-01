use crate::{protocol::bytes::Bytes, Error, Result};
use std::mem::size_of;

#[derive(Debug)]
pub struct WCursor<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

#[allow(dead_code)]
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
        let size = size_of::<u16>();
        let slice = self.slice(size)?;
        unsafe { Bytes::wbe_u16_unchecked(val, slice) };
        self.pos += size;
        Ok(())
    }

    #[inline]
    pub unsafe fn u16_be_unchecked(&mut self, val: u16) {
        Bytes::wbe_u16_unchecked(val, self.buf.get_unchecked_mut(self.pos..));
        self.pos += size_of::<u16>();
    }

    #[inline]
    pub fn u8(&mut self, val: u8) -> Result<()> {
        let size = size_of::<u8>();
        unsafe { *self.slice(size)?.get_unchecked_mut(0) = val };
        self.pos += size;
        Ok(())
    }

    #[inline]
    pub unsafe fn u8_unchecked(&mut self, val: u8) {
        *self.buf.get_unchecked_mut(self.pos) = val;
        self.pos += size_of::<u8>();
    }

    #[inline]
    pub fn bytes(&mut self, buf: &[u8]) -> Result<()> {
        let slice = self.slice(buf.len())?;
        slice.copy_from_slice(buf);
        self.pos += buf.len();
        Ok(())
    }

    #[inline]
    pub unsafe fn bytes_unchecked(&mut self, buf: &[u8]) {
        self.buf
            .get_unchecked_mut(self.pos..self.pos + buf.len())
            .copy_from_slice(buf);
        self.pos += buf.len();
    }
}
