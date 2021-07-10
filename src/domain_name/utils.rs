use crate::{
    constants::{DOMAIN_NAME_LABEL_MAX_LENGTH, DOMAIN_NAME_MAX_LENGTH},
    errors::{ProtocolError, ProtocolResult},
};

pub fn check_label_bytes(label: &[u8]) -> ProtocolResult<()> {
    if label.is_empty() {
        return Err(ProtocolError::DomainNameLabelMalformed);
    }

    let len = label.len();

    if len > DOMAIN_NAME_LABEL_MAX_LENGTH {
        return Err(ProtocolError::DomainNameLabelTooLong(len));
    }

    for b in label.iter().cloned() {
        if !(b.is_ascii_alphanumeric() || b == b'-') {
            return Err(ProtocolError::DomainNameLabelInvalidChar(b));
        }
    }

    // the slice is not empty (checked at the top of the function)
    // so it is sound to access it unchecked at the first and last bytes
    unsafe {
        if !label.get_unchecked(0).is_ascii_alphabetic() {
            return Err(ProtocolError::DomainNameLabelMalformed);
        }
        if !label.get_unchecked(len - 1).is_ascii_alphanumeric() {
            return Err(ProtocolError::DomainNameLabelMalformed);
        }
    }

    Ok(())
}

#[inline(always)]
pub fn check_label(label: &str) -> ProtocolResult<()> {
    check_label_bytes(label.as_bytes())
}

pub fn check_name_bytes(name: &[u8]) -> ProtocolResult<()> {
    if name.is_empty() {
        return Err(ProtocolError::DomainNameLabelMalformed);
    }

    // root domain name
    if name == b"." {
        return Ok(());
    }

    let len = name.len();
    let mut domain_start: Option<usize> = None;

    let mut i = 0;
    for j in 0..len {
        let byte = unsafe { *name.get_unchecked(j) };
        if byte == b'.' {
            let label = unsafe { name.get_unchecked(i..j) };
            check_label_bytes(label)?;
            i = j + 1;
            domain_start = Some(i);
        }
    }

    match domain_start {
        Some(ds) if len - ds > 0 => {
            let label = unsafe { name.get_unchecked(ds..len) };
            check_label_bytes(label)?;
        }
        None => check_label_bytes(name)?,
        _ => (),
    }

    let last_byte = unsafe { *name.get_unchecked(len - 1) };

    let effective_max_length = if last_byte == b'.' {
        DOMAIN_NAME_MAX_LENGTH - 1
    } else {
        DOMAIN_NAME_MAX_LENGTH - 2
    };

    if len > effective_max_length {
        return Err(ProtocolError::DomainNameTooLong);
    }

    Ok(())
}

#[inline(always)]
pub fn check_name(name: &str) -> ProtocolResult<()> {
    check_name_bytes(name.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_label() {
        let malformed: &[&[u8]] = &[b"", b"1abel", b"-abel", b"label-"];

        for m in malformed {
            let res = check_label_bytes(m);
            assert!(matches!(res, Err(ProtocolError::DomainNameLabelMalformed)));

            let res = check_label(std::str::from_utf8(m).unwrap());
            assert!(matches!(res, Err(ProtocolError::DomainNameLabelMalformed)));
        }

        let invalid_char: &[(&[u8], u8)] = &[(b"la.el", b'.'), (b"\tabel", b'\t')];
        for (ic, c) in invalid_char {
            let res = check_label_bytes(ic);
            assert!(matches!(
                res,
                Err(ProtocolError::DomainNameLabelInvalidChar(v)) if v == *c
            ));

            let res = check_label(std::str::from_utf8(ic).unwrap());
            assert!(matches!(
                res,
                Err(ProtocolError::DomainNameLabelInvalidChar(v)) if v == *c
            ));
        }

        let l_64 = "a".repeat(64);
        let too_large = &[l_64.as_bytes()];
        for tl in too_large {
            let res = check_label_bytes(tl);
            assert!(matches!(res, Err(ProtocolError::DomainNameLabelTooLong(l)) if l == tl.len()));

            let res = check_label(std::str::from_utf8(tl).unwrap());
            assert!(matches!(res, Err(ProtocolError::DomainNameLabelTooLong(l)) if l == tl.len()));
        }

        let l_63 = "a".repeat(63);
        let good: &[&[u8]] = &[b"label", b"labe1", l_63.as_bytes()];
        for g in good {
            assert!(check_label_bytes(g).is_ok());
            assert!(check_label(std::str::from_utf8(g).unwrap()).is_ok());
        }
    }

    #[test]
    fn test_check_name() {
        let good: &[&[u8]] = &[
            b".",
            b"com",
            b"example.com",
            b"exampl0.com.",
            b"exam-3le.com",
            b"su--b.exAmp1e.com",
        ];
        for g in good {
            assert!(check_name_bytes(g).is_ok());
            assert!(check_name(std::str::from_utf8(g).unwrap()).is_ok());
        }

        let malformed: &[&[u8]] = &[
            b"",
            b"..",
            b"3om",
            b"co-",
            b"example.com..",
            b"example..com",
            b"sub..example.com",
            b"1xample.com",
            b"example-.com",
            b"-xample.com",
        ];

        for m in malformed {
            let res = check_name_bytes(m);
            assert!(matches!(res, Err(ProtocolError::DomainNameLabelMalformed)));

            let res = check_name(std::str::from_utf8(m).unwrap());
            assert!(matches!(res, Err(ProtocolError::DomainNameLabelMalformed)));
        }

        let invalid_char: &[(&[u8], u8)] = &[(b"examp|e.com.", b'|')];

        for (ic, c) in invalid_char {
            let res = check_name_bytes(ic);
            assert!(matches!(
                res,
                Err(ProtocolError::DomainNameLabelInvalidChar(v)) if v == *c
            ));

            let res = check_name(std::str::from_utf8(ic).unwrap());
            assert!(matches!(
                res,
                Err(ProtocolError::DomainNameLabelInvalidChar(v)) if v == *c
            ));
        }

        let l_63 = "a".repeat(63);
        let l_61 = "b".repeat(61);
        let dn_253 = vec![l_63.clone(), l_63.clone(), l_63.clone()].join(".") + "." + l_61.as_str();
        let dn_254 = dn_253.clone() + "b";

        assert!(check_name_bytes(dn_253.as_str().as_bytes()).is_ok());
        assert!(check_name(dn_253.as_str()).is_ok());
        assert!(check_name_bytes((dn_253.clone() + ".").as_str().as_bytes()).is_ok());
        assert!(check_name((dn_253.clone() + ".").as_str()).is_ok());

        let too_long = &[dn_254.as_str()];
        for tl in too_long {
            let res = check_name(tl);
            assert!(matches!(res, Err(ProtocolError::DomainNameTooLong)));

            let res = check_name_bytes(tl.as_bytes());
            assert!(matches!(res, Err(ProtocolError::DomainNameTooLong)));
        }
    }
}
