use crate::{constants::Section, message::Header, Error, Result};

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

    pub fn next_section(&self) -> Option<Section> {
        if self.an_read < self.an_count {
            Some(Section::Answer)
        } else if self.ns_read < self.ns_count {
            Some(Section::Authority)
        } else if self.ar_read < self.ar_count {
            Some(Section::Additional)
        } else {
            None
        }
    }

    pub fn section_read(&mut self, section: Section) -> Result<()> {
        match section {
            Section::Answer => self.an_read += 1,
            Section::Authority => self.ns_read += 1,
            Section::Additional => self.ar_read += 1,
            _ => return Err(Error::BadSection(section)),
        }
        Ok(())
    }
}
