use crate::{
    constants::{DOMAIN_NAME_LABEL_MAX_LENGTH, DOMAIN_NAME_MAX_LENGTH},
    Error, Result,
};

pub fn check_label_bytes(label: &[u8]) -> Result<()> {
    if label.is_empty() {
        return Err(Error::DomainNameLabelIsEmpty);
    }

    let len = label.len();

    if len > DOMAIN_NAME_LABEL_MAX_LENGTH {
        return Err(Error::DomainNameLabelTooLong(len));
    }

    for b in label.iter().cloned() {
        if !(b.is_ascii_alphanumeric() || b == b'-') {
            return Err(Error::DomainNameLabelInvalidChar(
                "domain name label invalid character",
                b,
            ));
        }
    }

    // the slice is not empty (checked at the top of the function)
    // so it is sound to access it unchecked at the first and last bytes
    unsafe {
        let fc = label.get_unchecked(0);
        if !fc.is_ascii_alphanumeric() {
            return Err(Error::DomainNameLabelInvalidChar(
                "domain name label first character is not alphanumeric",
                *fc,
            ));
        }

        let lc = label.get_unchecked(len - 1);
        if !lc.is_ascii_alphanumeric() {
            return Err(Error::DomainNameLabelInvalidChar(
                "domain name label last character is not alphanumeric",
                *lc,
            ));
        }
    }

    Ok(())
}

#[inline(always)]
pub fn check_label(label: &str) -> Result<()> {
    check_label_bytes(label.as_bytes())
}

pub fn check_name_bytes(name: &[u8]) -> Result<()> {
    if name.is_empty() {
        return Err(Error::DomainNameLabelIsEmpty);
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

    let full_length = if last_byte == b'.' { len + 1 } else { len + 2 };

    if full_length > DOMAIN_NAME_MAX_LENGTH {
        return Err(Error::DomainNameTooLong(full_length));
    }

    Ok(())
}

#[inline(always)]
pub fn check_name(name: &str) -> Result<()> {
    check_name_bytes(name.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_label() {
        let res = check_label_bytes(b"");
        assert!(matches!(res, Err(Error::DomainNameLabelIsEmpty)));

        let malformed: &[(&[u8], u8)] = &[(b"-abel", b'-')];

        for (m, c) in malformed {
            let res = check_label_bytes(m);
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar(
                    "domain name label first character is not alphanumeric",
                    v
                )) if v == *c
            ));

            let res = check_label(std::str::from_utf8(m).unwrap());
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar(
                    "domain name label first character is not alphanumeric",
                    v
                )) if v == *c
            ));
        }

        let malformed: &[(&[u8], u8)] = &[(b"label-", b'-')];

        for (m, c) in malformed {
            let res = check_label_bytes(m);
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar(
                    "domain name label last character is not alphanumeric",
                    v
                )) if v == *c
            ));

            let res = check_label(std::str::from_utf8(m).unwrap());
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar(
                    "domain name label last character is not alphanumeric",
                    v
                )) if v == *c
            ));
        }

        let invalid_char: &[(&[u8], u8)] = &[(b"la.el", b'.'), (b"\tabel", b'\t')];
        for (ic, c) in invalid_char {
            let res = check_label_bytes(ic);
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar("domain name label invalid character", v)) if v == *c
            ));

            let res = check_label(std::str::from_utf8(ic).unwrap());
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar("domain name label invalid character", v)) if v == *c
            ));
        }

        let l_64 = "a".repeat(64);
        let too_large = &[l_64.as_bytes()];
        for tl in too_large {
            let res = check_label_bytes(tl);
            assert!(matches!(res, Err(Error::DomainNameLabelTooLong(l)) if l == tl.len()));

            let res = check_label(std::str::from_utf8(tl).unwrap());
            assert!(matches!(res, Err(Error::DomainNameLabelTooLong(l)) if l == tl.len()));
        }

        let l_63 = "a".repeat(63);
        let good: &[&[u8]] = &[b"label", b"labe1", b"1abel", l_63.as_bytes()];
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
            b"3om",
            b"example.com",
            b"exampl0.com.",
            b"3xample2.com",
            b"exam-3le.com",
            b"su--b.exAmp1e.com",
        ];
        for g in good {
            assert!(check_name_bytes(g).is_ok());
            assert!(check_name(std::str::from_utf8(g).unwrap()).is_ok());
        }

        let empty: &[&[u8]] = &[
            b"",
            b"..",
            b"example.com..",
            b"example..com",
            b"sub..example.com",
        ];
        for e in empty {
            let res = check_name_bytes(e);
            assert!(matches!(res, Err(Error::DomainNameLabelIsEmpty)));

            let res = check_name(std::str::from_utf8(e).unwrap());
            assert!(matches!(res, Err(Error::DomainNameLabelIsEmpty)));
        }

        let malformed: &[(&[u8], u8)] = &[(b"-xample.com", b'-')];

        for (m, c) in malformed {
            let res = check_name_bytes(m);
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar(
                    "domain name label first character is not alphanumeric",
                    v
                )) if v == *c
            ));

            let res = check_name(std::str::from_utf8(m).unwrap());
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar(
                    "domain name label first character is not alphanumeric",
                    v
                )) if v == *c
            ));
        }

        let malformed: &[(&[u8], u8)] = &[(b"co-", b'-'), (b"example-.com", b'-')];

        for (m, c) in malformed {
            let res = check_name_bytes(m);
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar(
                    "domain name label last character is not alphanumeric",
                    v
                )) if v == *c
            ));

            let res = check_name(std::str::from_utf8(m).unwrap());
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar(
                    "domain name label last character is not alphanumeric",
                    v
                )) if v == *c
            ));
        }

        let invalid_char: &[(&[u8], u8)] = &[(b"examp|e.com.", b'|')];

        for (ic, c) in invalid_char {
            let res = check_name_bytes(ic);
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar("domain name label invalid character", v)) if v == *c
            ));

            let res = check_name(std::str::from_utf8(ic).unwrap());
            assert!(matches!(
                res,
                Err(Error::DomainNameLabelInvalidChar("domain name label invalid character", v)) if v == *c
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
            assert!(matches!(res, Err(Error::DomainNameTooLong(s)) if s == tl.len() + 2));

            let res = check_name_bytes(tl.as_bytes());
            assert!(matches!(res, Err(Error::DomainNameTooLong(s)) if s == tl.len() + 2));
        }
    }
}
