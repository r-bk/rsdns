use crate::{
    bytes::Cursor,
    constants::DOMAIN_NAME_MAX_POINTERS,
    names::{self, reader, DName},
    Error, Result,
};

#[macro_use]
mod macros;

mod label;
pub use label::*;

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
    fn next_label(&mut self) -> Option<Result<Label<'a>>> {
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
    fn next_impl(&mut self) -> Result<Option<Label<'a>>> {
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
    type Item = Result<Label<'a>>;

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

pub(crate) fn skip_domain_name(c: &mut Cursor<'_>) -> Result<()> {
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
    Ok(())
}
