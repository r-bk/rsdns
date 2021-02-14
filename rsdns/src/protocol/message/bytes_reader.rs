use crate::{Result, RsDnsError};

pub struct BytesReader;

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
            Err(RsDnsError::EndOfBuffer)
        }
    }};
}

#[allow(clippy::missing_safety_doc)]
#[allow(dead_code)]
impl BytesReader {
    #[inline]
    pub unsafe fn u16_be_unchecked(buf: &[u8]) -> u16 {
        ru_be!(u16, buf)
    }

    #[inline]
    pub fn u16_be(buf: &[u8]) -> Result<u16> {
        r_be!(u16, buf)
    }

    #[inline]
    pub unsafe fn u32_be_unchecked(buf: &[u8]) -> u32 {
        ru_be!(u32, buf)
    }

    #[inline]
    pub fn u32_be(buf: &[u8]) -> Result<u32> {
        r_be!(u32, buf)
    }

    #[inline]
    pub unsafe fn i32_be_unchecked(buf: &[u8]) -> i32 {
        ru_be!(i32, buf)
    }

    #[inline]
    pub fn i32_be(buf: &[u8]) -> Result<i32> {
        r_be!(i32, buf)
    }
}
