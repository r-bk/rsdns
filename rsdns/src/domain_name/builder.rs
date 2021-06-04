use crate::{InlineName, Name, Result};

pub trait DomainNameBuilder {
    fn is_empty(&self) -> bool;
    fn set_root(&mut self);
    fn push_label_bytes(&mut self, label: &[u8]) -> Result<()>;
}

impl DomainNameBuilder for Name {
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
        self.push_label_bytes(label)
    }
}

impl DomainNameBuilder for InlineName {
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
        self.push_label_bytes(label)
    }
}
