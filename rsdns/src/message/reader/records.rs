use crate::{
    bytes::{Cursor, Reader, RrDataReader},
    constants::{MessageSection, RClass, RType},
    message::{reader::SectionTracker, Header},
    records::{self, ResourceRecord},
    Error, Result,
};
use std::convert::TryFrom;

/// An iterator over the resource record sections of a message.
///
/// Records are read from the [Answer](MessageSection::Answer),
/// [Authority](MessageSection::Authority) and [Additional](MessageSection::Additional)
/// message sections sequentially in this order. On every iteration a single resource record
/// is read and returned together with its corresponding section type.
/// Unknown resource record types are silently skipped.
///
/// Memory is allocated only for those records which contain dynamically allocated fields in the
/// record data.
pub struct Records<'a> {
    cursor: Cursor<'a>,
    section_tracker: SectionTracker,
    err: bool,
}

macro_rules! rrr {
    ($self:ident, $rr:ident, $pos:ident, $rclass:ident, $ttl:ident, $rdlen:ident) => {{
        ResourceRecord::$rr(records::$rr {
            name: $self.cursor.clone_with_pos($pos).read()?,
            rclass: $rclass,
            $ttl,
            data: $self.cursor.read_rr_data($rdlen)?,
        })
    }};
}

impl<'a> Records<'a> {
    pub(crate) fn new(cursor: Cursor<'a>, header: &Header) -> Records<'a> {
        Records {
            cursor,
            section_tracker: SectionTracker::new(header),
            err: false,
        }
    }

    fn read(&mut self) -> Option<Result<(MessageSection, ResourceRecord)>> {
        if !self.err {
            let res = self.read_impl();
            if res.is_ok() {
                Some(res)
            } else if let Err(Error::IterationStop) = res {
                None
            } else {
                self.err = true;
                Some(res)
            }
        } else {
            None
        }
    }

    fn read_impl(&mut self) -> Result<(MessageSection, ResourceRecord)> {
        loop {
            let section = self.section_tracker.next_section();

            if let Some(section) = section {
                let domain_name_pos = self.cursor.pos();
                self.cursor.skip_domain_name()?;

                let rtype = self.cursor.u16_be()?;
                let rclass = self.cursor.u16_be()?;
                let ttl = self.cursor.u32_be()?;
                let rdlen = self.cursor.u16_be()? as usize;

                let rclass = match RClass::try_from(rclass) {
                    Ok(rc) => rc,
                    _ => {
                        /* unsupported RCLASS */
                        self.cursor.skip(rdlen)?;
                        self.section_tracker.section_read(section)?;
                        continue;
                    }
                };

                let rtype = match RType::try_from(rtype) {
                    Ok(rt) => rt,
                    _ => {
                        /* unsupported RTYPE */
                        self.cursor.skip(rdlen)?;
                        self.section_tracker.section_read(section)?;
                        continue;
                    }
                };

                let rec = match rtype {
                    RType::A => rrr!(self, A, domain_name_pos, rclass, ttl, rdlen),
                    RType::NS => rrr!(self, Ns, domain_name_pos, rclass, ttl, rdlen),
                    RType::MD => rrr!(self, Md, domain_name_pos, rclass, ttl, rdlen),
                    RType::MF => rrr!(self, Mf, domain_name_pos, rclass, ttl, rdlen),
                    RType::CNAME => rrr!(self, Cname, domain_name_pos, rclass, ttl, rdlen),
                    RType::SOA => rrr!(self, Soa, domain_name_pos, rclass, ttl, rdlen),
                    RType::MB => rrr!(self, Mb, domain_name_pos, rclass, ttl, rdlen),
                    RType::MG => rrr!(self, Mg, domain_name_pos, rclass, ttl, rdlen),
                    RType::MR => rrr!(self, Mr, domain_name_pos, rclass, ttl, rdlen),
                    RType::NULL => rrr!(self, Null, domain_name_pos, rclass, ttl, rdlen),
                    RType::WKS => rrr!(self, Wks, domain_name_pos, rclass, ttl, rdlen),
                    RType::PTR => rrr!(self, Ptr, domain_name_pos, rclass, ttl, rdlen),
                    RType::HINFO => rrr!(self, Hinfo, domain_name_pos, rclass, ttl, rdlen),
                    RType::MINFO => rrr!(self, Minfo, domain_name_pos, rclass, ttl, rdlen),
                    RType::MX => rrr!(self, Mx, domain_name_pos, rclass, ttl, rdlen),
                    RType::TXT => rrr!(self, Txt, domain_name_pos, rclass, ttl, rdlen),
                    RType::AAAA => rrr!(self, Aaaa, domain_name_pos, rclass, ttl, rdlen),
                };

                self.section_tracker.section_read(section)?;
                break Ok((section, rec));
            } else {
                break Err(Error::IterationStop);
            }
        }
    }
}

impl Iterator for Records<'_> {
    type Item = Result<(MessageSection, ResourceRecord)>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}
