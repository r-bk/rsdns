macro_rules! r_be {
    ($self:ident, $t:ty) => {{
        if $self.len() >= std::mem::size_of::<$t>() {
            let buf = unsafe { $self.buf.get_unchecked($self.pos..) };
            let ptr = buf.as_ptr() as *const $t;
            let v = unsafe { ptr.read_unaligned() };
            $self.pos += std::mem::size_of::<$t>();
            Ok(v.to_be())
        } else {
            Err($self.bound_error())
        }
    }};
}

macro_rules! ru_be {
    ($self:ident, $t:ty) => {{
        debug_assert!($self.len() >= std::mem::size_of::<$t>());
        let buf = $self.buf.get_unchecked($self.pos..);
        let ptr = buf.as_ptr() as *const $t;
        let v = ptr.read_unaligned();
        $self.pos += std::mem::size_of::<$t>();
        v.to_be()
    }};
}

cfg_any_client! {
    macro_rules! wu_be {
        ($self:ident, $t:ty, $val:ident) => {{
            debug_assert!($self.len() >= std::mem::size_of::<$t>());
            let buf = $self.buf.get_unchecked_mut($self.pos..);
            let ptr = buf.as_mut_ptr() as *mut $t;
            ptr.write_unaligned($val.to_be());
            $self.pos += std::mem::size_of::<$t>();
        }};
    }

    macro_rules! w_be {
        ($self:ident, $t:ty, $val:ident) => {{
            if $self.len() >= std::mem::size_of::<$t>() {
                let buf = unsafe { $self.buf.get_unchecked_mut($self.pos..) };
                let ptr = buf.as_mut_ptr() as *mut $t;
                unsafe { ptr.write_unaligned($val.to_be()) };
                $self.pos += std::mem::size_of::<$t>();
                Ok(())
            } else {
                Err(Error::BufferTooShort(std::mem::size_of::<$t>()))
            }
        }};
    }
}
