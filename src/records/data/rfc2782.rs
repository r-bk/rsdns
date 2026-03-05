use crate::{
    Result,
    bytes::{Cursor, Reader, RrDataReader},
    names::Name,
    records::Type,
};

/// Service locator.
///
/// [RFC 2782](https://www.rfc-editor.org/rfc/rfc2782.html)
#[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
pub struct Srv {
    /// The priority of this target host.
    /// A client MUST attempt to contact the target host with the lowest-numbered priority
    /// it can reach; target hosts with the same priority SHOULD be tried in an order defined
    /// by the weight field.
    pub priority: u16,

    /// A server selection mechanism.
    /// The weight field specifies a relative weight for entries with the same priority.
    /// Larger weights SHOULD be given a proportionately higher probability of being selected.
    pub weight: u16,

    /// The port on this target host of this service.
    pub port: u16,

    /// The domain name of the target host.
    /// A target of "." means that the service is decidedly not available at this domain.
    pub target: Name,
}

rr_data!(Srv, Type::SRV);

impl RrDataReader<Srv> for Cursor<'_> {
    fn read_rr_data(&mut self, rd_len: usize) -> Result<Srv> {
        self.window(rd_len)?;
        let rr = Ok(Srv {
            priority: self.u16_be()?,
            weight: self.u16_be()?,
            port: self.u16_be()?,
            target: self.read()?,
        });
        self.close_window()?;
        rr
    }
}
