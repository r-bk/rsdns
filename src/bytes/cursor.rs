use crate::{errors::ProtocolError, ProtocolResult};

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    buf: &'a [u8],
    pos: usize,
    orig: Option<&'a [u8]>,
}

impl<'a> Cursor<'a> {
    #[inline]
    pub const fn new(buf: &[u8]) -> Cursor {
        Cursor {
            buf,
            pos: 0,
            orig: None,
        }
    }

    #[inline]
    pub const fn with_pos(buf: &[u8], pos: usize) -> Cursor {
        Cursor {
            buf,
            pos,
            orig: None,
        }
    }

    #[inline]
    pub fn clone_with_pos(&self, pos: usize) -> Cursor {
        Cursor {
            buf: self.buf,
            pos,
            orig: None,
        }
    }

    pub fn window(&mut self, size: usize) -> ProtocolResult<()> {
        if self.orig.is_none() {
            if self.len() >= size {
                self.orig = Some(self.buf);
                self.buf = unsafe { self.buf.get_unchecked(..self.pos + size) };
                Ok(())
            } else {
                Err(ProtocolError::EndOfBuffer)
            }
        } else {
            Err(ProtocolError::CursorAlreadyInWindow)
        }
    }

    pub fn close_window(&mut self) -> ProtocolResult<()> {
        if self.orig.is_some() {
            if self.pos == self.buf.len() {
                self.buf = self.orig.take().unwrap();
                Ok(())
            } else {
                Err(ProtocolError::CursorWindowError {
                    window_end: self.buf.len(),
                    pos: self.pos,
                })
            }
        } else {
            Err(ProtocolError::CursorNotInWindow)
        }
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos
    }

    pub fn skip(&mut self, distance: usize) -> ProtocolResult<()> {
        if self.len() >= distance {
            self.pos += distance;
            Ok(())
        } else {
            Err(self.bound_error())
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

    pub fn u16_be(&mut self) -> ProtocolResult<u16> {
        r_be!(self, u16)
    }

    pub unsafe fn u16_be_unchecked(&mut self) -> u16 {
        ru_be!(self, u16)
    }

    pub fn u32_be(&mut self) -> ProtocolResult<u32> {
        r_be!(self, u32)
    }

    pub fn u128_be(&mut self) -> ProtocolResult<u128> {
        r_be!(self, u128)
    }

    pub fn u8(&mut self) -> ProtocolResult<u8> {
        if !self.is_empty() {
            let v = unsafe { *self.buf.get_unchecked(self.pos) };
            self.pos += 1;
            Ok(v)
        } else {
            Err(self.bound_error())
        }
    }

    pub fn slice(&mut self, size: usize) -> ProtocolResult<&[u8]> {
        if self.len() >= size {
            let pos = self.pos;
            self.pos += size;
            Ok(unsafe { self.buf.get_unchecked(pos..pos + size) })
        } else {
            Err(self.bound_error())
        }
    }

    #[inline]
    fn bound_error(&self) -> ProtocolError {
        if self.orig.is_none() {
            ProtocolError::EndOfBuffer
        } else {
            ProtocolError::EndOfWindow
        }
    }
}
