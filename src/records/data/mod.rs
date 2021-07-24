//! Resource record data.

#[macro_use]
mod macros;

mod rfc1035;
pub use rfc1035::*;

mod rfc3596;
pub use rfc3596::*;

mod rdata;
pub use rdata::*;

/// Enumerates supported resource records' data.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RecordData {
    /// A host address (IPv4).
    A(rfc1035::A),
    /// An authoritative name server.
    Ns(rfc1035::Ns),
    /// A mail destination.
    Md(rfc1035::Md),
    /// A mail forwarder.
    Mf(rfc1035::Mf),
    /// The canonical name for an alias.
    Cname(rfc1035::Cname),
    /// Marks the start of a zone of authority.
    Soa(rfc1035::Soa),
    /// A mailbox domain name.
    Mb(rfc1035::Mb),
    /// A mail group member.
    Mg(rfc1035::Mg),
    /// A mail rename domain name.
    Mr(rfc1035::Mr),
    /// The Null record.
    Null(rfc1035::Null),
    /// A well known service description.
    Wks(rfc1035::Wks),
    /// A domain name pointer.
    Ptr(rfc1035::Ptr),
    /// Host information.
    Hinfo(rfc1035::Hinfo),
    /// Mailbox or mail list information.
    Minfo(rfc1035::Minfo),
    /// Mail exchange.
    Mx(rfc1035::Mx),
    /// Text strings.
    Txt(rfc1035::Txt),
    /// A host address (IPv6)
    Aaaa(rfc3596::Aaaa),
}
