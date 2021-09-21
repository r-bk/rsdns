use crate::{Error, Result};
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct CSize(pub(crate) u16);

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    buf: *const u8,
    capacity: CSize,
    pos: CSize,
    orig_capacity: CSize,
    _marker: PhantomData<&'a [u8]>,
}

impl<'a> Cursor<'a> {
    #[inline]
    pub const fn new(buf: &'a [u8]) -> Result<Cursor<'a>> {
        Self::with_pos(buf, CSize(0))
    }

    #[inline]
    pub const fn with_pos(buf: &'a [u8], pos: CSize) -> Result<Cursor<'a>> {
        if buf.len() > u16::MAX as usize {
            return Err(Error::BadParam("message buffer cannot exceed 65535 bytes"));
        }

        Ok(Cursor {
            buf: buf.as_ptr(),
            capacity: CSize(buf.len() as u16),
            pos,
            orig_capacity: CSize(0),
            _marker: PhantomData,
        })
    }

    #[inline]
    pub fn clone_with_pos(&self, pos: CSize) -> Cursor<'a> {
        let mut c = self.clone();
        c.pos = pos;
        c
    }

    pub fn window(&mut self, size: CSize) -> Result<()> {
        if self.orig_capacity.0 == 0 {
            if self.len().0 >= size.0 {
                self.orig_capacity = self.capacity;
                self.capacity = CSize(self.pos.0 + size.0);
                Ok(())
            } else {
                Err(Error::EndOfBuffer)
            }
        } else {
            Err(Error::CursorAlreadyInWindow)
        }
    }

    pub fn close_window(&mut self) -> Result<()> {
        if self.orig_capacity.0 != 0 {
            if self.pos.0 == self.capacity.0 {
                self.capacity = self.orig_capacity;
                self.orig_capacity = CSize(0);
                Ok(())
            } else {
                Err(Error::CursorWindowError {
                    window_end: self.capacity.0 as usize,
                    pos: self.pos.0 as usize,
                })
            }
        } else {
            Err(Error::CursorNotInWindow)
        }
    }

    #[inline]
    pub fn pos(&self) -> CSize {
        self.pos
    }

    #[inline]
    pub fn set_pos(&mut self, pos: CSize) {
        self.pos = pos
    }

    pub fn skip(&mut self, distance: CSize) -> Result<()> {
        if self.len().0 >= distance.0 {
            self.pos.0 += distance.0;
            Ok(())
        } else {
            Err(self.bound_error())
        }
    }

    pub fn len(&self) -> CSize {
        if self.pos.0 < self.capacity.0 {
            CSize(self.capacity.0 - self.pos.0)
        } else {
            CSize(0)
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len().0 == 0
    }

    pub fn u16_be(&mut self) -> Result<u16> {
        r_be!(self, u16)
    }

    pub unsafe fn u16_be_unchecked(&mut self) -> u16 {
        ru_be!(self, u16)
    }

    pub fn u32_be(&mut self) -> Result<u32> {
        r_be!(self, u32)
    }

    pub fn u128_be(&mut self) -> Result<u128> {
        r_be!(self, u128)
    }

    pub fn u8(&mut self) -> Result<u8> {
        if !self.is_empty() {
            let v = unsafe {
                let ptr = self.buf.add(self.pos.0 as usize);
                ptr.read_unaligned()
            };
            self.pos.0 += 1;
            Ok(v)
        } else {
            Err(self.bound_error())
        }
    }

    pub fn slice(&mut self, size: CSize) -> Result<&'a [u8]> {
        if self.len().0 >= size.0 {
            let pos = self.pos;
            self.pos.0 += size.0;
            Ok(
                unsafe {
                    std::slice::from_raw_parts(self.buf.add(pos.0 as usize), size.0 as usize)
                },
            )
        } else {
            Err(self.bound_error())
        }
    }

    #[inline]
    fn bound_error(&self) -> Error {
        if self.orig_capacity.0 == 0 {
            Error::EndOfBuffer
        } else {
            Error::EndOfWindow
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_cursor_size() {
        let buf = [0u8; 32];
        let cursor = Cursor::new(&buf[..]).unwrap();
        assert_eq!(std::mem::size_of_val(&cursor), 16);
    }
}
