macro_rules! labels_loop {
    ($cursor:expr, $max_pos:expr, $n_pointers:expr, $done:expr, $dn:expr, $f:ident, $l:ident) => {
        loop {
            let pos = $cursor.pos();
            let label = $cursor.u8()?;
            if label == 0 {
                if $max_pos.0 == 0 {
                    $max_pos = $cursor.pos();
                }
                $done = true;
                $f!();
            } else if is_length(label) {
                let bytes = $cursor.slice(CSize(label as u16))?;
                $l!(bytes, pos, $dn);
            } else if is_pointer(label) {
                let o2 = $cursor.u8()?;
                let offset = CSize(pointer_to_offset(label, o2));

                if $max_pos.0 == 0 {
                    $max_pos = $cursor.pos();
                }
                if offset.0 >= $max_pos.0 - 2 {
                    return Err(Error::DomainNameBadPointer {
                        pointer: offset.0 as usize,
                        max_offset: $max_pos.0 as usize,
                    });
                }
                $n_pointers += 1;
                if $n_pointers > DOMAIN_NAME_MAX_POINTERS {
                    return Err(Error::DomainNameTooMuchPointers);
                }
                $cursor.set_pos(offset);
            } else {
                return Err(Error::DomainNameBadLabelType(label));
            }
        }
    };
}

macro_rules! return_none {
    () => {
        return Ok(None);
    };
}

macro_rules! return_false {
    () => {
        return Ok(false);
    };
}

macro_rules! return_label {
    ($bytes:ident, $pos:ident, $dn:expr) => {
        let _ = $dn;
        names::check_label_bytes($bytes)?;
        return Ok(Some(LabelRef { $bytes, $pos }));
    };
}

macro_rules! return_true {
    ($bytes:ident, $pos:ident, $dn:expr) => {
        let _ = $pos;
        let _ = $dn;
        names::check_label_bytes($bytes)?;
        return Ok(true);
    };
}

macro_rules! append_label {
    ($bytes:ident, $pos:ident, $dn:expr) => {
        let _ = $pos;
        $dn.append_label_bytes($bytes)?;
    };
}

macro_rules! check_label {
    ($bytes:ident, $pos:ident, $dn:expr) => {
        let _ = $pos;
        let _ = $dn;
        names::check_label_bytes($bytes)?;
    };
}

macro_rules! break_loop {
    () => {
        break;
    };
}
