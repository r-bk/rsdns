use crate::{
    bytes::{Cursor, Reader},
    constants::DOMAIN_NAME_MAX_POINTERS,
    names::{self, DName},
    Error, Result,
};

#[macro_use]
mod macros;

mod label_ref;
pub use label_ref::*;

#[cfg(test)]
mod test_labels;

const POINTER_MASK: u8 = 0b1100_0000;
const LENGTH_MASK: u8 = 0b0011_1111;

/// Iterator over encoded domain name labels.
#[derive(Debug)]
pub struct Labels<'a> {
    cursor: Cursor<'a>,
    n_pointers: usize,
    max_pos: usize,
    done: bool,
}

#[allow(dead_code)]
impl<'a> Labels<'a> {
    #[inline]
    pub(crate) fn new(c: Cursor<'a>) -> Labels<'a> {
        Self {
            cursor: c,
            n_pointers: 0,
            max_pos: 0,
            done: false,
        }
    }

    #[inline]
    pub(crate) fn max_pos(&self) -> usize {
        self.max_pos
    }

    #[inline]
    fn next_label(&mut self) -> Option<Result<LabelRef<'a>>> {
        if self.done {
            return None;
        }

        let res = self.next_impl();
        match res {
            Ok(Some(l)) => Some(Ok(l)),
            Ok(None) => None,
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }

    #[inline]
    pub(crate) fn skip_next_label(&mut self) -> Option<Result<()>> {
        if self.done {
            return None;
        }

        let res = self.skip_impl();
        match res {
            Ok(true) => Some(Ok(())),
            Ok(false) => None,
            Err(e) => {
                self.done = true;
                Some(Err(e))
            }
        }
    }

    #[inline]
    fn next_impl(&mut self) -> Result<Option<LabelRef<'a>>> {
        labels_loop!(
            self.cursor,
            self.max_pos,
            self.n_pointers,
            self.done,
            (),
            return_none,
            return_label
        )
    }

    #[inline]
    fn skip_impl(&mut self) -> Result<bool> {
        labels_loop!(
            self.cursor,
            self.max_pos,
            self.n_pointers,
            self.done,
            (),
            return_false,
            return_true
        )
    }
}

impl<'a> Iterator for Labels<'a> {
    type Item = Result<LabelRef<'a>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_label()
    }
}

pub(crate) fn read_domain_name<N: DName>(c: &mut Cursor<'_>) -> Result<N> {
    let mut dn = N::default();
    let mut cursor = c.clone();
    let mut max_pos = 0;
    let mut n_pointers = 0;
    #[allow(unused_assignments)]
    let mut done = false;

    labels_loop!(
        cursor,
        max_pos,
        n_pointers,
        done,
        dn,
        break_loop,
        append_label
    );

    let _ = done; // make clippy happy

    if dn.is_empty() {
        dn.set_root();
    }
    c.set_pos(max_pos);
    Ok(dn)
}

pub(crate) fn skip_domain_name(c: &mut Cursor<'_>) -> Result<usize> {
    let start = c.pos();
    let mut cursor = c.clone();
    let mut max_pos = 0;
    let mut n_pointers = 0;
    #[allow(unused_assignments)]
    let mut done = false;

    labels_loop!(
        cursor,
        max_pos,
        n_pointers,
        done,
        (),
        break_loop,
        check_label
    );

    let _ = done; // make clippy happy

    c.set_pos(max_pos);
    Ok(c.pos() - start)
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

impl<N> Reader<N> for Cursor<'_>
where
    N: DName,
{
    #[inline]
    fn read(&mut self) -> Result<N> {
        read_domain_name(self)
    }
}

impl Cursor<'_> {
    #[inline]
    pub fn skip_domain_name(&mut self) -> Result<usize> {
        skip_domain_name(self)
    }

    #[inline]
    pub fn skip_question(&mut self) -> Result<()> {
        skip_domain_name(self)?;
        self.skip(4) // qtype(2) + qclass(2)
    }

    #[inline]
    pub fn skip_rr(&mut self) -> Result<()> {
        skip_domain_name(self)?;
        self.skip(8)?; // Type(2) + Class(2) + TTL(4)
        let rd_len = self.u16_be()?;
        self.skip(rd_len as usize)
    }
}
