use crate::{
    bytes::{Cursor, Reader, RrDataReader},
    constants::{RClass, RType, RecordsSection},
    errors::{Error, ProtocolError},
    message::{reader::SectionTracker, Header, RecordType},
    records::{data::RecordData, ResourceRecord},
    Result,
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
/// record data.
///
/// Returns:
///
/// - `Some(Ok((`[`RecordsSection`]`, `[`ResourceRecord`]`)))` - if a record was read successfully
/// - `Some(Err(_))` - on error
/// - `None` - if there is nothing left to read, or a previous call resulted in error
///
/// # Examples
///
/// ```rust
/// use rsdns::{
///     constants::RecordsSection,
///     message::reader::MessageReader,
///     records::data::RecordData,
/// };
///
/// fn print_addresses(buf: &[u8]) -> rsdns::Result<()> {
///     let mr = MessageReader::new(buf)?;
///
///     for result in mr.records() {
///         let (section, record) = result?;
///
///         if section != RecordsSection::Answer {
///             // skip addresses in sections after Answer
///             break;
///         }
///
///         match record.rdata {
///             RecordData::A(ref rdata) => {
///                 println!(
///                     "Name: {}; Class: {}; TTL: {}; ipv4: {}",
///                     record.name, record.rclass, record.ttl, rdata.address
///                 );
///             }
///             RecordData::Aaaa(ref rdata) => {
///                 println!(
///                     "Name: {}; Class: {}; TTL: {}; ipv6: {}",
///                     record.name, record.rclass, record.ttl, rdata.address
///                 );
///             }
///             _ => continue,
///         }
///     }
///
///     Ok(())
/// }
///
/// ```
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
            rtype: RType::$rr,
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
            let section = self.section_tracker.next_section();

            if let Some(section) = section {
                let domain_name_pos = self.cursor.pos();
                self.cursor.skip_domain_name()?;

                let rtype: RecordType = self.cursor.u16_be()?.into();
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
                    RType::Ns => rrr!(self, Ns, domain_name_pos, rclass, ttl, rdlen),
                    RType::Md => rrr!(self, Md, domain_name_pos, rclass, ttl, rdlen),
                    RType::Mf => rrr!(self, Mf, domain_name_pos, rclass, ttl, rdlen),
                    RType::Cname => rrr!(self, Cname, domain_name_pos, rclass, ttl, rdlen),
                    RType::Soa => rrr!(self, Soa, domain_name_pos, rclass, ttl, rdlen),
                    RType::Mb => rrr!(self, Mb, domain_name_pos, rclass, ttl, rdlen),
                    RType::Mg => rrr!(self, Mg, domain_name_pos, rclass, ttl, rdlen),
                    RType::Mr => rrr!(self, Mr, domain_name_pos, rclass, ttl, rdlen),
                    RType::Null => rrr!(self, Null, domain_name_pos, rclass, ttl, rdlen),
                    RType::Wks => rrr!(self, Wks, domain_name_pos, rclass, ttl, rdlen),
                    RType::Ptr => rrr!(self, Ptr, domain_name_pos, rclass, ttl, rdlen),
                    RType::Hinfo => rrr!(self, Hinfo, domain_name_pos, rclass, ttl, rdlen),
                    RType::Minfo => rrr!(self, Minfo, domain_name_pos, rclass, ttl, rdlen),
                    RType::Mx => rrr!(self, Mx, domain_name_pos, rclass, ttl, rdlen),
                    RType::Txt => rrr!(self, Txt, domain_name_pos, rclass, ttl, rdlen),
                    RType::Aaaa => rrr!(self, Aaaa, domain_name_pos, rclass, ttl, rdlen),
                    RType::Axfr | RType::Mailb | RType::Maila | RType::Any => {
                        return Err(Error::ProtocolError(ProtocolError::UnexpectedRType(rtype)));
                    }
                };

                self.section_tracker.section_read(section)?;
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
