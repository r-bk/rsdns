use crate::{
    bytes::{Cursor, Reader, RrDataReader},
    constants::Type,
    names::Name,
    Result,
};
use std::net::Ipv4Addr;

// ------------------------------------------------------------------------------------------------

/// A host address (IPv4).
///
/// [RFC 1035 section 3.4.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.4.1)
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct A {
    /// Internet address (IPv4).
    pub address: Ipv4Addr,
}

rr_data!(A);

impl RrDataReader<A> for Cursor<'_> {
    fn read_rr_data(&mut self, rd_len: usize) -> Result<A> {
        self.window(rd_len)?;
        let rr = Ok(A {
            address: self.read()?,
        });
        self.close_window()?;
        rr
    }
}

// ------------------------------------------------------------------------------------------------

rr_dn_data!(
    /// The canonical name for an alias.
    ///
    /// [RFC 1035 section 3.3.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.1)
    Cname,
    /// A domain name which specifies the canonical or primary name for the owner.
    /// The RR owner name is an alias.
    cname
);

// ------------------------------------------------------------------------------------------------

/// Host information.
///
/// Standard values for CPU and OS can be found in
/// [RFC 1010](https://www.rfc-editor.org/rfc/rfc1010.html).
///
/// [RFC 1035 section 3.3.2](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.2)
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
pub struct Hinfo {
    /// A character-string which specifies the CPU type.
    pub cpu: Vec<u8>,

    /// A character-string which specifies the operating system type.
    pub os: Vec<u8>,
}

rr_data!(Hinfo);

impl RrDataReader<Hinfo> for Cursor<'_> {
    fn read_rr_data(&mut self, rd_len: usize) -> Result<Hinfo> {
        self.window(rd_len)?;
        let rr = Ok(Hinfo {
            cpu: self.read_character_string()?,
            os: self.read_character_string()?,
        });
        self.close_window()?;
        rr
    }
}

// ------------------------------------------------------------------------------------------------

/// A well known service description.
///
/// [RFC 1035 section 3.4.2](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.4.2)
#[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub struct Wks {
    /// Host address.
    pub address: Ipv4Addr,

    /// Protocol number.
    pub protocol: u8,

    /// Variable length bitmap.
    ///
    /// Has one bit per port of the specified protocol. The first bit corresponds to port 0,
    /// the second to port 1, etc. If the bit map does not include a bit for a
    /// protocol of interest, that bit is assumed zero.
    ///
    /// The appropriate values and mnemonics for ports and protocols are specified in
    /// [RFC 1010](https://www.rfc-editor.org/rfc/rfc1010.html).
    pub bitmap: Vec<u8>,
}

rr_data!(Wks);

impl RrDataReader<Wks> for Cursor<'_> {
    fn read_rr_data(&mut self, rd_len: usize) -> Result<Wks> {
        self.window(rd_len)?;
        let rr = Ok(Wks {
            address: self.read()?,
            protocol: self.u8()?,
            bitmap: Vec::from(self.slice(rd_len - 5)?),
        });
        self.close_window()?;
        rr
    }
}

// ------------------------------------------------------------------------------------------------

rr_dn_data!(
    /// A mailbox domain name.
    ///
    /// [RFC 1035 section 3.3.3](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.3)
    Mb,
    /// A domain name which specifies a host which has the specified mailbox.
    madname
);

// ------------------------------------------------------------------------------------------------

rr_dn_data!(
    /// A mail destination.
    ///
    /// Obsolete.
    ///
    /// [RFC 1035 section 3.3.4](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.4)
    Md,
    /// A domain name which specifies a host which has a mail agent for the domain which should
    /// be able to deliver mail for the domain.
    madname
);

// ------------------------------------------------------------------------------------------------

rr_dn_data!(
    /// A mail forwarder.
    ///
    /// Obsolete.
    ///
    /// [RFC 1035 section 3.3.5](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.5)
    Mf,
    /// A domain name which specifies a host which has a mail agent for the domain which will
    /// accept mail for forwarding to the domain.
    madname
);

// ------------------------------------------------------------------------------------------------

rr_dn_data!(
    /// A mail group member.
    ///
    /// [RFC 1035 section 3.3.6](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.6)
    Mg,
    /// A domain name which specifies a mailbox which is a member of the mail group specified
    /// by the domain name.
    mgmname
);

// ------------------------------------------------------------------------------------------------

/// Mailbox or mail list information.
///
/// [RFC 1035 section 3.3.7](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.7)
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
pub struct Minfo {
    /// A domain name which specifies a mailbox which is responsible for the mailing list
    /// or mailbox.  If this domain name names the root, the owner of the MINFO RR is
    /// responsible for itself.
    pub rmailbx: Name,

    /// A domain name which specifies a mailbox which is to receive error messages related
    /// to the mailing list or mailbox specified by the owner of the MINFO RR. If this domain
    /// name names the root, errors should be returned to the sender of the message.
    pub emailbx: Name,
}

rr_data!(Minfo);

impl RrDataReader<Minfo> for Cursor<'_> {
    fn read_rr_data(&mut self, rd_len: usize) -> Result<Minfo> {
        self.window(rd_len)?;
        let rr = Ok(Minfo {
            rmailbx: self.read()?,
            emailbx: self.read()?,
        });
        self.close_window()?;
        rr
    }
}

// ------------------------------------------------------------------------------------------------

rr_dn_data!(
    /// A mail rename domain name.
    ///
    /// [RFC 1035 section 3.3.8](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.8)
    Mr,
    /// A domain name which specifies a mailbox which is the proper rename of the specified mailbox.
    newname
);

// ------------------------------------------------------------------------------------------------

/// Mail exchange.
///
/// [RFC 1035 section 3.3.9](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.9)
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
pub struct Mx {
    /// Specifies the preference given to this RR among others at the same owner.
    /// Lower values are preferred.
    pub preference: u16,
    /// A domain name which specifies a host willing to act as a mail exchange for the owner name.
    pub exchange: Name,
}

rr_data!(Mx);

impl RrDataReader<Mx> for Cursor<'_> {
    fn read_rr_data(&mut self, rd_len: usize) -> Result<Mx> {
        self.window(rd_len)?;
        let rr = Ok(Mx {
            preference: self.u16_be()?,
            exchange: self.read()?,
        });
        self.close_window()?;
        rr
    }
}

// ------------------------------------------------------------------------------------------------

/// The Null record.
///
/// [RFC 1035 section 3.3.10](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.10)
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
pub struct Null {
    /// Anything at all may be in the RDATA field.
    pub anything: Vec<u8>,
}

rr_data!(Null);

impl RrDataReader<Null> for Cursor<'_> {
    fn read_rr_data(&mut self, rd_len: usize) -> Result<Null> {
        self.window(rd_len)?;
        let rr = Ok(Null {
            anything: Vec::from(self.slice(rd_len)?),
        });
        self.close_window()?;
        rr
    }
}

// ------------------------------------------------------------------------------------------------

rr_dn_data!(
    /// An authoritative name server.
    ///
    /// [RFC 1035 section 3.3.11](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.11)
    Ns,
    /// A domain name  which specifies a host which should be authoritative for the
    /// specified class and domain.
    nsdname
);

// ------------------------------------------------------------------------------------------------

rr_dn_data!(
    /// A domain name pointer.
    ///
    /// [RFC 1035 section 3.3.12](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.12)
    Ptr,
    /// A domain name which points to some location in the domain name space.
    ptrdname
);

// ------------------------------------------------------------------------------------------------

/// Marks the start of a zone of authority.
///
/// [RFC 1035 section 3.3.13](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.13)
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
pub struct Soa {
    /// The domain name of the name server that was the original or primary source of
    /// data for this zone.
    pub mname: Name,
    /// A domain name which specifies the mailbox of the person responsible for this zone.
    pub rname: Name,
    /// The version number of the original copy of the zone. Zone transfers preserve this value.
    /// This value wraps and should be compared using sequence space arithmetic.
    pub serial: u32,
    /// A time interval before the zone should be refreshed.
    pub refresh: u32,
    /// A time interval that should elapse before a failed refresh should be retried.
    pub retry: u32,
    /// A time value that specifies the upper limit on the time interval that can elapse before
    /// the zone is no longer authoritative.
    pub expire: u32,
    /// Minimum TTL field that should be exported with any RR from this zone.
    pub minimum: u32,
}

rr_data!(Soa);

impl RrDataReader<Soa> for Cursor<'_> {
    fn read_rr_data(&mut self, rd_len: usize) -> Result<Soa> {
        self.window(rd_len)?;
        let rr = Ok(Soa {
            mname: self.read()?,
            rname: self.read()?,
            serial: self.u32_be()?,
            refresh: self.u32_be()?,
            retry: self.u32_be()?,
            expire: self.u32_be()?,
            minimum: self.u32_be()?,
        });
        self.close_window()?;
        rr
    }
}

// ------------------------------------------------------------------------------------------------

/// Text strings.
///
/// [RFC 1035 section 3.3.14](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.3.14)
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
pub struct Txt {
    /// TXT RRs are used to hold descriptive text. The semantics of the text
    /// depends on the domain where it is found.
    pub text: Vec<u8>,
}

rr_data!(Txt);

impl RrDataReader<Txt> for Cursor<'_> {
    fn read_rr_data(&mut self, mut rd_len: usize) -> Result<Txt> {
        self.window(rd_len)?;
        let mut text = Vec::with_capacity(rd_len);
        while rd_len > 0 {
            let len = self.u8()? as usize;
            if len > 0 {
                text.extend_from_slice(self.slice(len)?);
            }
            rd_len -= len + 1;
        }
        self.close_window()?;
        Ok(Txt { text })
    }
}
