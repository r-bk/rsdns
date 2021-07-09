use crate::{constants::RecordsSection, message::Header, Result};

#[derive(Debug, Clone, Default)]
pub struct SectionTracker {
    an_count: u16,
    an_read: u16,
    ns_count: u16,
    ns_read: u16,
    ar_count: u16,
    ar_read: u16,
}

impl SectionTracker {
    pub fn new(header: &Header) -> Self {
        Self {
            an_count: header.an_count,
            ns_count: header.ns_count,
            ar_count: header.ar_count,
            ..Default::default()
        }
    }

    pub fn next_section(&self) -> Option<RecordsSection> {
        if self.an_read < self.an_count {
            Some(RecordsSection::Answer)
        } else if self.ns_read < self.ns_count {
            Some(RecordsSection::Authority)
        } else if self.ar_read < self.ar_count {
            Some(RecordsSection::Additional)
        } else {
            None
        }
    }

    pub fn section_read(&mut self, section: RecordsSection) -> Result<()> {
        match section {
            RecordsSection::Answer => self.an_read += 1,
            RecordsSection::Authority => self.ns_read += 1,
            RecordsSection::Additional => self.ar_read += 1,
        }
        Ok(())
    }
}
