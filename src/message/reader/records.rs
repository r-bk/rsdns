use crate::{
    bytes::{Cursor, Reader, RrDataReader},
    constants::{Class, Type},
    message::{reader::SectionTracker, ClassValue, Header, RecordsSection, TypeValue},
    records::{data::RecordData, ResourceRecord},
    Error, Result,
};
use std::convert::TryFrom;

/// An iterator over the resource record sections of a message.
///
/// Records are read from the [Answer](RecordsSection::Answer),
/// [Authority](RecordsSection::Authority) and [Additional](RecordsSection::Additional)
/// message sections sequentially in this order. On every iteration a single resource record
/// is read and returned together with its corresponding section type.
/// Unknown resource record types are silently skipped.
///
/// Memory is allocated only for those records which contain dynamically allocated fields in the
/// record data. In particular, reading A and AAAA records does not involve memory allocation.
///
/// # Returns
///
/// - `Some(Ok((`[`RecordsSection`]`, `[`ResourceRecord`]`)))` - if a record was read successfully
/// - `Some(Err(_))` - on error
/// - `None` - if there is nothing left to read, or a previous call resulted in error
///
/// # Examples
///
/// See [`MessageIterator`] for an example.
///
/// [`MessageIterator`]: crate::message::reader::MessageIterator
pub struct Records<'a> {
    cursor: Cursor<'a>,
    section_tracker: SectionTracker,
    err: bool,
}

macro_rules! rrr {
    ($self:ident, $rr:ident, $pos:ident, $rclass:ident, $ttl:ident, $rdlen:ident) => {{
        ResourceRecord {
            name: $self.cursor.clone_with_pos($pos).read()?,
            rclass: $rclass,
            rtype: Type::$rr,
            $ttl,
            rdata: RecordData::$rr($self.cursor.read_rr_data($rdlen)?),
        }
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

    fn read(&mut self) -> Option<Result<(RecordsSection, ResourceRecord)>> {
        if !self.err {
            let res = self.read_impl();
            match res {
                Ok(Some(t)) => Some(Ok(t)),
                Ok(None) => None,
                Err(e) => {
                    self.err = true;
                    Some(Err(e))
                }
            }
        } else {
            None
        }
    }

    fn read_impl(&mut self) -> Result<Option<(RecordsSection, ResourceRecord)>> {
        loop {
            let section = self.section_tracker.next_section(self.cursor.pos());

            if let Some(section) = section {
                let domain_name_pos = self.cursor.pos();
                self.cursor.skip_domain_name()?;

                let rtype: TypeValue = self.cursor.u16_be()?.into();
                let rclass: ClassValue = self.cursor.u16_be()?.into();
                let ttl = self.cursor.u32_be()?;
                let rdlen = self.cursor.u16_be()? as usize;

                let rclass = match Class::try_from(rclass) {
                    Ok(rc) => rc,
                    _ => {
                        /* unsupported RCLASS */
                        self.cursor.skip(rdlen)?;
                        self.section_tracker
                            .section_read(section, self.cursor.pos());
                        continue;
                    }
                };

                let rtype = match Type::try_from(rtype) {
                    Ok(rt) if rt != Type::Opt => rt,
                    _ => {
                        // unsupported RTYPE or OPT. OPT record is supported in MessageReader only
                        self.cursor.skip(rdlen)?;
                        self.section_tracker
                            .section_read(section, self.cursor.pos());
                        continue;
                    }
                };

                let rec = match rtype {
                    Type::A => rrr!(self, A, domain_name_pos, rclass, ttl, rdlen),
                    Type::Ns => rrr!(self, Ns, domain_name_pos, rclass, ttl, rdlen),
                    Type::Md => rrr!(self, Md, domain_name_pos, rclass, ttl, rdlen),
                    Type::Mf => rrr!(self, Mf, domain_name_pos, rclass, ttl, rdlen),
                    Type::Cname => rrr!(self, Cname, domain_name_pos, rclass, ttl, rdlen),
                    Type::Soa => rrr!(self, Soa, domain_name_pos, rclass, ttl, rdlen),
                    Type::Mb => rrr!(self, Mb, domain_name_pos, rclass, ttl, rdlen),
                    Type::Mg => rrr!(self, Mg, domain_name_pos, rclass, ttl, rdlen),
                    Type::Mr => rrr!(self, Mr, domain_name_pos, rclass, ttl, rdlen),
                    Type::Null => rrr!(self, Null, domain_name_pos, rclass, ttl, rdlen),
                    Type::Wks => rrr!(self, Wks, domain_name_pos, rclass, ttl, rdlen),
                    Type::Ptr => rrr!(self, Ptr, domain_name_pos, rclass, ttl, rdlen),
                    Type::Hinfo => rrr!(self, Hinfo, domain_name_pos, rclass, ttl, rdlen),
                    Type::Minfo => rrr!(self, Minfo, domain_name_pos, rclass, ttl, rdlen),
                    Type::Mx => rrr!(self, Mx, domain_name_pos, rclass, ttl, rdlen),
                    Type::Txt => rrr!(self, Txt, domain_name_pos, rclass, ttl, rdlen),
                    Type::Aaaa => rrr!(self, Aaaa, domain_name_pos, rclass, ttl, rdlen),
                    Type::Axfr | Type::Mailb | Type::Maila | Type::Any | Type::Opt => {
                        return Err(Error::UnexpectedType(rtype));
                    }
                };

                self.section_tracker
                    .section_read(section, self.cursor.pos());
                break Ok(Some((section, rec)));
            } else {
                break Ok(None);
            }
        }
    }
}

impl Iterator for Records<'_> {
    type Item = Result<(RecordsSection, ResourceRecord)>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}
