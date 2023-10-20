use crate::{
    bytes::{Cursor, Reader, RrDataReader},
    message::{reader::SectionTracker, Header, RecordsSection},
    records::{data::RecordData, Class, ResourceRecord, Type},
    Error, Result,
};

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
    ($self:ident, $rt:expr, $rr:ident, $pos:ident, $rclass:ident, $ttl:ident, $rdlen:ident) => {{
        ResourceRecord {
            name: $self.cursor.clone_with_pos($pos).read()?,
            rclass: $rclass,
            rtype: $rt,
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

                let rtype: Type = self.cursor.u16_be()?.into();
                let rclass: Class = self.cursor.u16_be()?.into();
                let ttl = self.cursor.u32_be()?;
                let rdlen = self.cursor.u16_be()? as usize;

                if !rclass.is_defined() {
                    /* unsupported RCLASS */
                    self.cursor.skip(rdlen)?;
                    self.section_tracker
                        .section_read(section, self.cursor.pos());
                    continue;
                }

                if !rtype.is_defined() {
                    // unsupported RTYPE or OPT. OPT record is supported in MessageReader only
                    self.cursor.skip(rdlen)?;
                    self.section_tracker
                        .section_read(section, self.cursor.pos());
                    continue;
                }

                let rec = match rtype {
                    Type::A => rrr!(self, Type::A, A, domain_name_pos, rclass, ttl, rdlen),
                    Type::NS => rrr!(self, Type::NS, Ns, domain_name_pos, rclass, ttl, rdlen),
                    Type::MD => rrr!(self, Type::MD, Md, domain_name_pos, rclass, ttl, rdlen),
                    Type::MF => rrr!(self, Type::MF, Mf, domain_name_pos, rclass, ttl, rdlen),
                    Type::CNAME => rrr!(
                        self,
                        Type::CNAME,
                        Cname,
                        domain_name_pos,
                        rclass,
                        ttl,
                        rdlen
                    ),
                    Type::SOA => rrr!(self, Type::SOA, Soa, domain_name_pos, rclass, ttl, rdlen),
                    Type::MB => rrr!(self, Type::MB, Mb, domain_name_pos, rclass, ttl, rdlen),
                    Type::MG => rrr!(self, Type::MG, Mg, domain_name_pos, rclass, ttl, rdlen),
                    Type::MR => rrr!(self, Type::MR, Mr, domain_name_pos, rclass, ttl, rdlen),
                    Type::NULL => rrr!(self, Type::NULL, Null, domain_name_pos, rclass, ttl, rdlen),
                    Type::WKS => rrr!(self, Type::WKS, Wks, domain_name_pos, rclass, ttl, rdlen),
                    Type::PTR => rrr!(self, Type::PTR, Ptr, domain_name_pos, rclass, ttl, rdlen),
                    Type::HINFO => rrr!(
                        self,
                        Type::HINFO,
                        Hinfo,
                        domain_name_pos,
                        rclass,
                        ttl,
                        rdlen
                    ),
                    Type::MINFO => rrr!(
                        self,
                        Type::MINFO,
                        Minfo,
                        domain_name_pos,
                        rclass,
                        ttl,
                        rdlen
                    ),
                    Type::MX => rrr!(self, Type::MX, Mx, domain_name_pos, rclass, ttl, rdlen),
                    Type::TXT => rrr!(self, Type::TXT, Txt, domain_name_pos, rclass, ttl, rdlen),
                    Type::AAAA => rrr!(self, Type::AAAA, Aaaa, domain_name_pos, rclass, ttl, rdlen),
                    _ => {
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
