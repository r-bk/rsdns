use crate::{protocol::DomainName, Result};

pub trait DomainNameBuilder {
    fn is_empty(&self) -> bool;
    fn set_root(&mut self);
    fn push_label_bytes(&mut self, label: &[u8]) -> Result<()>;
}

impl DomainNameBuilder for String {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline(always)]
    fn set_root(&mut self) {
        self.clear();
        self.push('.');
    }

    fn push_label_bytes(&mut self, label: &[u8]) -> Result<()> {
        DomainName::check_label_bytes(label)?;

        // at this point the label is proven to be valid,
        // which means it is sound to convert it unchecked as a valid label is ASCII
        let label_as_str = unsafe { std::str::from_utf8_unchecked(label) };

        self.push_str(label_as_str);
        self.push('.');
        Ok(())
    }
}

impl DomainNameBuilder for DomainName {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    #[inline(always)]
    fn set_root(&mut self) {
        self.set_root();
    }

    #[inline(always)]
    fn push_label_bytes(&mut self, label: &[u8]) -> Result<()> {
        DomainName::push_label_bytes(self, label)
    }
}
