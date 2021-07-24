use crate::{
    constants::{RClass, RCode, RType, RecordsSection},
    errors::{AnswerError, Error, Result},
    message::{reader::MessageReader, MessageType},
    records::{
        data::{RData, RecordData},
        RecordSet, ResourceRecord,
    },
    Name,
};
use std::convert::TryFrom;

/// A query answer.
///
/// [Answer] is the struct returned from the resolver's `query` method. It contains an answer to
/// data-type queries. Meta-queries like [RType::Any] do not return an [Answer].
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Answer<D: RData> {
    pub(crate) cnames: Vec<Name>,
    pub(crate) rrset: RecordSet<D>,
}

impl<D: RData> Answer<D> {
    /// Returns the CNAME chain of the resulting RRset.
    ///
    /// A chain may appear in a response message if the corresponding query name has a CNAME record.
    /// The domain name in that record may have a CNAME record of its own,
    /// creating a chain of domain names. The resulting RRset is derived from records of the
    /// last domain name in the chain.
    #[inline]
    pub fn cnames(&self) -> &Vec<Name> {
        &self.cnames
    }

    /// Returns the RRset with the queried record data.
    ///
    /// Usually the RRset will belong to the query name. However, in case the query name has a CNAME
    /// record, the RRset will belong to the last domain name in the CNAME chain.
    #[inline]
    pub fn rrset(&self) -> &RecordSet<D> {
        &self.rrset
    }

    /// Parses an Answer from a message.
    pub fn from_msg(msg: &[u8]) -> Result<Self> {
        let mr = MessageReader::new(msg)?;

        let flags = mr.header().flags;

        if flags.message_type() != MessageType::Response {
            return Err(Error::AnswerError(AnswerError::BadMessageType(
                flags.message_type(),
            )));
        }

        if flags.response_code() != RCode::NoError {
            return Err(Error::AnswerError(AnswerError::BadResponseCode(
                flags.response_code(),
            )));
        }

        if flags.truncated() {
            return Err(Error::AnswerError(AnswerError::MessageTruncated));
        }

        let question = mr.question()?;
        let mut records = Self::read_answer_records(&mr)?;

        let rtype = RType::try_from(question.qtype)?;
        if rtype != D::RTYPE {
            return Err(Error::AnswerError(AnswerError::NoAnswer));
        }

        let rclass = RClass::try_from(question.qclass)?;
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
                        return Err(Error::AnswerError(AnswerError::NoAnswer));
                    }
                }
            }
        };

        rrset.name = name;
        Ok(Self { cnames, rrset })
    }

    fn extract_rrset(
        records: &mut Vec<Option<ResourceRecord>>,
        name: &Name,
        rclass: RClass,
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
        rclass: RClass,
    ) -> Option<ResourceRecord> {
        #[allow(clippy::manual_flatten)]
        for o in records.iter_mut() {
            if let Some(r) = o {
                if r.name == name && r.rtype == RType::Cname && r.rclass == rclass {
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
