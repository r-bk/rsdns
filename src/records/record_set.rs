use crate::{
    Error, Result,
    message::{
        MessageType, RCode, RecordsSection,
        reader::{MessageReader, NameRef, RecordHeaderRef},
    },
    names::Name,
    records::{Class, Opt, Type, data::RData},
};

/// A set of similar records.
///
/// [`RecordSet`] (or RRset) is a set of resource records with the same name, class and type,
/// but with different data.
///
/// Defined in:
/// - [RFC 2181 section 5](https://www.rfc-editor.org/rfc/rfc2181#section-5)
/// - [RFC 7719 section 4](https://www.rfc-editor.org/rfc/rfc7719.html#section-4)
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RecordSet<D: RData> {
    /// The name all records in this set belong to.
    pub name: Name,

    /// The class of records in this set.
    pub rclass: Class,

    /// The TTL of records in this set.
    ///
    /// In case an RRSet contains records with different TTL, this is the minimum among them.
    pub ttl: u32,

    /// The various record data of this set.
    pub rdata: Vec<D>,
}

impl<D: RData> RecordSet<D> {
    /// Record type as associated constant.
    pub const RTYPE: Type = D::RTYPE;

    /// Parses a [`RecordSet`] from a response message.
    ///
    /// This method performs *CNAME flattening*, which is the process of traversing a *chain* of
    /// CNAME records until requested record set is found.
    ///
    /// A *CNAME chain* may occur in a DNS message if the question name has a [`CNAME`] record
    /// pointing to its canonical name. The canonical name may have a [`CNAME`] record of its own,
    /// creating a *chain*. The record set belongs to the last name in the *chain*,
    /// which is reflected in the returned record set's [`name`](RecordSet::name) attribute.
    ///
    /// [`CNAME`]: crate::records::Type::CNAME
    pub fn from_msg(msg: &[u8]) -> Result<Self> {
        let mut mr = MessageReader::new(msg)?;
        let header = mr.header()?;

        let flags = header.flags;
        if flags.message_type() != MessageType::Response {
            return Err(Error::BadMessageType(flags.message_type()));
        }

        if flags.truncated() {
            return Err(Error::MessageTruncated);
        }

        let question = mr.the_question_ref()?;
        let mut headers = Self::read_answer_headers(&mut mr)?;
        let opt = Self::read_opt(&mut mr)?;

        let response_code = if let Some(ref o) = opt {
            RCode::extended(header.flags.response_code(), o.rcode_extension())
        } else {
            header.flags.response_code()
        };

        if response_code != RCode::NOERROR {
            return Err(Error::BadResponseCode(response_code));
        }

        let rclass = question.qclass;
        let mut name = question.qname;

        let mut rrset = loop {
            match Self::extract_rrset(&mr, &mut headers, &name, rclass)? {
                Some(rrset) => break rrset,
                None => {
                    if let Some(n) = Self::extract_cname(&mr, &mut headers, &name, rclass)? {
                        name = n;
                    } else {
                        return Err(Error::NoAnswer);
                    }
                }
            }
        };

        rrset.name = Name::try_from(name)?;
        Ok(rrset)
    }

    #[inline(always)]
    fn extract_rrset<'m, 'a: 'm>(
        mr: &'m MessageReader<'a>,
        headers: &mut [Option<RecordHeaderRef<'a>>],
        name: &NameRef<'a>,
        rclass: Class,
    ) -> Result<Option<RecordSet<D>>> {
        let mut rrset = RecordSet {
            name: Name::default(),
            rclass,
            ttl: u32::MAX,
            rdata: Vec::<D>::default(),
        };

        #[allow(clippy::manual_flatten)]
        for o in headers.iter_mut() {
            if let Some(h) = o {
                if h.name().eq(name)? && h.rtype() == D::RTYPE && h.rclass() == rclass {
                    rrset.ttl = rrset.ttl.min(h.ttl());
                    rrset.rdata.push(mr.record_data_at::<D>(h.marker())?);
                    o.take();
                }
            }
        }

        if !rrset.rdata.is_empty() {
            Ok(Some(rrset))
        } else {
            Ok(None)
        }
    }

    #[inline(always)]
    fn extract_cname<'m, 'a: 'm>(
        mr: &'m MessageReader<'a>,
        headers: &mut [Option<RecordHeaderRef<'a>>],
        name: &NameRef<'a>,
        rclass: Class,
    ) -> Result<Option<NameRef<'a>>> {
        #[allow(clippy::manual_flatten)]
        for o in headers.iter_mut() {
            if let Some(h) = o {
                if h.name().eq(name)? && h.rtype() == Type::CNAME && h.rclass() == rclass {
                    let n = mr.name_ref_at(h.marker());
                    o.take();
                    return Ok(Some(n));
                }
            }
        }
        Ok(None)
    }

    #[inline(always)]
    fn read_answer_headers<'m, 'a: 'm>(
        mr: &'m mut MessageReader<'a>,
    ) -> Result<Vec<Option<RecordHeaderRef<'a>>>> {
        let mut headers = Vec::with_capacity(mr.records_count_in(RecordsSection::Answer));
        while mr.has_records_in(RecordsSection::Answer) {
            let header = mr.record_header_ref()?;
            mr.skip_record_data(header.marker())?;
            headers.push(Some(header));
        }
        Ok(headers)
    }

    #[inline(always)]
    fn read_opt(mr: &mut MessageReader) -> Result<Option<Opt>> {
        let mut opt = None;
        while mr.has_records() {
            let marker = mr.record_marker()?;
            if marker.rtype == Type::OPT {
                opt = Some(mr.opt_record(&marker)?);
                break;
            } else {
                mr.skip_record_data(&marker)?;
            }
        }
        Ok(opt)
    }
}
