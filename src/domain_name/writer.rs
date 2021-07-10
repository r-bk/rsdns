use crate::{
    bytes::WCursor,
    constants::DOMAIN_NAME_MAX_LENGTH,
    errors::{ProtocolError, ProtocolResult},
};

impl WCursor<'_> {
    #[inline]
    fn write_label(&mut self, label: &[u8]) -> ProtocolResult<()> {
        super::check_label_bytes(label)?;
        if self.len() > label.len() {
            unsafe {
                self.u8_unchecked(label.len() as u8);
                self.bytes_unchecked(label);
            }
            Ok(())
        } else {
            Err(ProtocolError::BufferTooShort(self.pos() + label.len() + 1))
        }
    }

    pub fn write_domain_name(&mut self, name: &str) -> ProtocolResult<usize> {
        self.write_domain_name_bytes(name.as_bytes())
    }

    pub fn write_domain_name_bytes(&mut self, name: &[u8]) -> ProtocolResult<usize> {
        if name.is_empty() {
            return Err(ProtocolError::DomainNameLabelIsEmpty);
        }

        let start = self.pos();
        let len = name.len();

        let mut i = 0;
        let mut domain_start = None;

        for j in 0..len {
            let byte = unsafe { *name.get_unchecked(j) };
            if byte == b'.' {
                let label = unsafe { name.get_unchecked(i..j) };
                self.write_label(label)?;
                i = j + 1;
                domain_start = Some(i);
            }
        }

        match domain_start {
            Some(ds) if len - ds > 0 => {
                let label = unsafe { name.get_unchecked(ds..len) };
                self.write_label(label)?;
            }
            None => self.write_label(name)?,
            _ => {}
        };

        self.u8(0)?;

        let length = self.pos() - start;
        if length > DOMAIN_NAME_MAX_LENGTH {
            return Err(ProtocolError::DomainNameTooLong);
        }

        Ok(length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bytes::{Cursor, Reader},
        InlineName,
    };
    use std::str::FromStr;

    #[test]
    fn test_write_good_flow() {
        let expectations: Vec<(&str, &[u8])> = vec![
            ("sub.example.com", b"\x03sub\x07example\x03com\x00"),
            ("example.com.", b"\x07example\x03com\x00"),
            ("com", b"\x03com\x00"),
            ("com.", b"\x03com\x00"),
        ];

        for ex in expectations {
            let mut arr = [0xFFu8; 64];
            let mut wcursor = WCursor::new(&mut arr[..]);

            let len = wcursor.write_domain_name(ex.0).unwrap();

            assert_eq!(len, ex.1.len());
            assert_eq!(&arr[..len], ex.1);

            let mut cursor = Cursor::new(&arr[..len]);
            let dn: InlineName = cursor.read().unwrap();

            assert_eq!(dn, InlineName::from_str(ex.0).unwrap());
        }
    }

    #[test]
    fn test_write_domain_too_long() {
        let mut arr = [0xFFu8; 1024];

        let l_63 = "a".repeat(63);

        let long_label = vec![l_63.as_str(), l_63.as_str(), l_63.as_str(), l_63.as_str()].join(".");
        assert_eq!(long_label.len(), 255);

        {
            let mut wcursor = WCursor::new(&mut arr[..]);
            assert!(matches!(
                wcursor.write_domain_name(&long_label),
                Err(ProtocolError::DomainNameTooLong)
            ));
        }

        {
            let mut wcursor = WCursor::new(&mut arr[..]);
            assert!(matches!(
                wcursor.write_domain_name(&long_label[..long_label.len() - 1]),
                Err(ProtocolError::DomainNameTooLong)
            ));
        }

        {
            let mut wcursor = WCursor::new(&mut arr[..]);
            let len = wcursor
                .write_domain_name(&long_label[..long_label.len() - 2])
                .unwrap();
            assert_eq!(len, 255);

            let mut cursor = Cursor::new(&arr[..len]);
            let dn: InlineName = cursor.read().unwrap();

            assert_eq!(
                dn,
                InlineName::from_str(&long_label[..long_label.len() - 2]).unwrap()
            );
            assert_eq!(dn.len(), 254);
        }
    }

    #[test]
    fn test_write_malformed_label() {
        let empty: Vec<&str> = vec![
            "",
            "..",
            "example.com..",
            "example..com",
            "sub..example.com",
        ];

        for e in empty {
            let mut arr = [0xFFu8; 32];
            let mut wcursor = WCursor::new(&mut arr[..]);
            assert!(matches!(
                wcursor.write_domain_name(e),
                Err(ProtocolError::DomainNameLabelIsEmpty)
            ));
        }

        let samples: Vec<&str> = vec!["3om", "co-", "1xample.com", "example-.com", "-xample.com"];

        for s in samples {
            let mut arr = [0xFFu8; 32];
            let mut wcursor = WCursor::new(&mut arr[..]);
            assert!(matches!(
                wcursor.write_domain_name(s),
                Err(ProtocolError::DomainNameLabelMalformed)
            ));
        }
    }
}
