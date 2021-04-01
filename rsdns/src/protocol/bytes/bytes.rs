use crate::{Error, Result};

pub struct Bytes;

macro_rules! ru_be {
    ($t:ty, $buf:ident) => {{
        debug_assert!($buf.len() >= std::mem::size_of::<$t>());
        let ptr = $buf.as_ptr() as *const $t;
        let v = ptr.read_unaligned();
        v.to_be()
    }};
}

macro_rules! r_be {
    ($t:ty, $buf:ident) => {{
        if $buf.len() >= std::mem::size_of::<$t>() {
            let ptr = $buf.as_ptr() as *const $t;
            let v = unsafe { ptr.read_unaligned() };
            Ok(v.to_be())
        } else {
            Err(Error::EndOfBuffer)
        }
    }};
}

macro_rules! wu_be {
    ($val:ident, $t:ty, $buf:ident) => {{
        debug_assert!($buf.len() >= std::mem::size_of::<$t>());
        let ptr = $buf.as_mut_ptr() as *mut $t;
        ptr.write_unaligned($val.to_be());
    }};
}

macro_rules! w_be {
    ($val:ident, $t:ty, $buf:ident) => {{
        if $buf.len() >= std::mem::size_of::<$t>() {
            let ptr = $buf.as_mut_ptr() as *mut $t;
            unsafe { ptr.write_unaligned($val.to_be()) };
            Ok(())
        } else {
            Err(Error::BufferTooShort(std::mem::size_of::<$t>()))
        }
    }};
}

#[allow(dead_code)]
impl Bytes {
    #[inline]
    pub unsafe fn rbe_u16_unchecked(buf: &[u8]) -> u16 {
        ru_be!(u16, buf)
    }

    #[inline]
    pub unsafe fn wbe_u16_unchecked(val: u16, buf: &mut [u8]) {
        wu_be!(val, u16, buf);
    }

    #[inline]
    pub fn rbe_u16(buf: &[u8]) -> Result<u16> {
        r_be!(u16, buf)
    }

    #[inline]
    pub fn wbe_u16(val: u16, buf: &mut [u8]) -> Result<()> {
        w_be!(val, u16, buf)
    }

    #[inline]
    pub unsafe fn rbe_u32_unchecked(buf: &[u8]) -> u32 {
        ru_be!(u32, buf)
    }

    #[inline]
    pub unsafe fn wbe_u32_unchecked(val: u32, buf: &mut [u8]) {
        wu_be!(val, u32, buf);
    }

    #[inline]
    pub fn rbe_u32(buf: &[u8]) -> Result<u32> {
        r_be!(u32, buf)
    }

    #[inline]
    pub fn wbe_u32(val: u32, buf: &mut [u8]) -> Result<()> {
        w_be!(val, u32, buf)
    }

    #[inline]
    pub unsafe fn rbe_i32_unchecked(buf: &[u8]) -> i32 {
        ru_be!(i32, buf)
    }

    #[inline]
    pub unsafe fn wbe_i32_unchecked(val: i32, buf: &mut [u8]) {
        wu_be!(val, i32, buf);
    }

    #[inline]
    pub fn rbe_i32(buf: &[u8]) -> Result<i32> {
        r_be!(i32, buf)
    }

    #[inline]
    pub fn wbe_i32(val: i32, buf: &mut [u8]) -> Result<()> {
        w_be!(val, i32, buf)
    }
}
