use crate::{
    constants::{Class, RCode, RecordsSection, Type},
    message::{reader::MessageReader, MessageType},
    records::{
        data::{RData, RecordData},
        ResourceRecord,
    },
    Error, Name, Result,
};
use std::convert::TryFrom;

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
    /// [`CNAME`]: crate::constants::Type::Cname
    pub fn from_msg(msg: &[u8]) -> Result<Self> {
        let mr = MessageReader::new(msg)?;

        let flags = mr.header().flags;

        if flags.message_type() != MessageType::Response {
            return Err(Error::BadMessageType(flags.message_type()));
        }

        if flags.response_code() != RCode::NoError {
            return Err(Error::BadResponseCode(flags.response_code()));
        }

        if flags.truncated() {
            return Err(Error::MessageTruncated);
        }

        let question = mr.question()?;
        let mut records = Self::read_answer_records(&mr)?;

        let rclass = Class::try_from(question.qclass)?;
        let mut name = Name::from(&question.qname);
        let mut cnames = Vec::new();

        let mut rrset = loop {
            match Self::extract_rrset(&mut records, &name, rclass)? {
                Some(rrset) => break rrset,
                None => {
                    if let Some(cname_rec) = Self::extract_cname(&mut records, &name, rclass) {
                        match cname_rec.rdata {
                            RecordData::Cname(s) => {
                                cnames.push(s.cname.clone());
                                name = s.cname;
                            }
                            _ => {
                                // should not get here
                                return Err(Error::InternalError(
                                    "unexpected RecordData discriminant: Cname expected",
                                ));
                            }
                        }
                    } else {
                        return Err(Error::NoAnswer);
                    }
                }
            }
        };

        rrset.name = name;
        Ok(rrset)
    }

    fn extract_rrset(
        records: &mut Vec<Option<ResourceRecord>>,
        name: &Name,
        rclass: Class,
    ) -> Result<Option<RecordSet<D>>> {
        let mut rrset = RecordSet {
            name: Name::default(),
            rclass,
            ttl: u32::MAX,
            rdata: Vec::<D>::default(),
        };

        #[allow(clippy::manual_flatten)]
        for o in records.iter_mut() {
            if let Some(r) = o {
                if r.name == *name && r.rtype == D::RTYPE && r.rclass == rclass {
                    rrset.ttl = rrset.ttl.min(r.ttl);
                    let rec = o.take().unwrap(); // o.is_some() == true, so no panic here
                    rrset.rdata.push(D::from(rec.rdata)?);
                }
            }
        }

        if !rrset.rdata.is_empty() {
            Ok(Some(rrset))
        } else {
            Ok(None)
        }
    }

    fn extract_cname(
        records: &mut Vec<Option<ResourceRecord>>,
        name: &Name,
        rclass: Class,
    ) -> Option<ResourceRecord> {
        #[allow(clippy::manual_flatten)]
        for o in records.iter_mut() {
            if let Some(r) = o {
                if r.name == name && r.rtype == Type::Cname && r.rclass == rclass {
                    return o.take();
                }
            }
        }
        None
    }

    fn read_answer_records(mr: &MessageReader) -> Result<Vec<Option<ResourceRecord>>> {
        let mut records = Vec::new();
        for res in mr.records() {
            let (section, record) = res?;
            if section == RecordsSection::Answer {
                records.push(Some(record));
            } else {
                // Answer is the first section. Skip the rest.
                break;
            }
        }
        Ok(records)
    }
}
