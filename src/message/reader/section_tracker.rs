use crate::{constants::RecordsSection, message::Header};

#[derive(Debug, Clone, Default)]
struct Counts {
    total: u16,
    read: u16,
}

#[derive(Debug, Clone, Default)]
pub struct SectionTracker {
    qd: Counts,
    sections: [Counts; 3],
    offsets: [u16; 3],
}

impl SectionTracker {
    pub fn new(header: &Header) -> Self {
        Self {
            qd: Counts {
                total: header.qd_count,
                read: 0,
            },
            sections: [
                Counts {
                    total: header.an_count,
                    read: 0,
                },
                Counts {
                    total: header.ns_count,
                    read: 0,
                },
                Counts {
                    total: header.ar_count,
                    read: 0,
                },
            ],
            ..Default::default()
        }
    }

    #[inline]
    fn section(&mut self, section: RecordsSection) -> &mut Counts {
        &mut self.sections[section as usize]
    }

    pub fn set(&mut self, header: &Header) {
        self.qd.total = header.qd_count;
        self.section(RecordsSection::Answer).total = header.an_count;
        self.section(RecordsSection::Authority).total = header.ns_count;
        self.section(RecordsSection::Additional).total = header.ar_count;
    }

    pub fn next_section(&mut self, pos: usize) -> Option<RecordsSection> {
        for s in RecordsSection::VALUES {
            let s_num = s as usize;
            let counts = &self.sections[s_num];
            if counts.read < counts.total {
                if counts.read == 0 && self.offsets[s_num] == 0 {
                    // This is the first record in this section
                    self.offsets[s_num] = pos as u16;
                }
                // Set the offset of previous sections, if they are empty.
                for p in (0..s_num).rev() {
                    if self.offsets[p] == 0 && self.sections[p].total == 0 {
                        self.offsets[p] = pos as u16;
                    } else {
                        break;
                    }
                }
                return Some(s);
            }
        }
        None
    }

    pub fn section_offset(&self, section: RecordsSection) -> Option<usize> {
        if self.offsets[section as usize] != 0 {
            Some(self.offsets[section as usize] as usize)
        } else {
            None
        }
    }

    pub fn seek(&mut self, section: RecordsSection) {
        for s in RecordsSection::VALUES {
            let counts = self.section(s);
            if s < section {
                counts.read = counts.total;
            } else {
                counts.read = 0;
            }
        }
    }

    pub fn section_read(&mut self, section: RecordsSection, pos: usize) {
        let counts = self.section(section);
        counts.read += 1;

        if counts.total == counts.read {
            // This is the last record in this section.
            // Set the offset of the next section, if needed.
            // If the next section is empty, continue until a non-empty section is found.
            let n_num = section as usize + 1;
            for n in n_num..3 {
                if self.offsets[n] == 0 {
                    self.offsets[n] = pos as u16;
                    if self.sections[n].total != 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    pub fn question_read(&mut self, pos: usize) {
        self.qd.read += 1;
        if self.qd.total == self.qd.read {
            // This is the last question.
            // Set the offset of the Answers section.
            // Continue as long as the next section is empty.
            for n in 0..3 {
                if self.offsets[n] == 0 {
                    self.offsets[n] = pos as u16;
                    if self.sections[n].total != 0 {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }

    pub fn records_left(&self) -> usize {
        self.sections
            .iter()
            .fold(0, |acc, c| acc + (c.total - c.read) as usize)
    }

    pub fn records_left_in(&self, section: RecordsSection) -> usize {
        let counts = &self.sections[section as usize];
        (counts.total - counts.read) as usize
    }

    pub fn questions_left(&self) -> usize {
        (self.qd.total - self.qd.read) as usize
    }
}
