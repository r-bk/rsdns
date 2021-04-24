mod opcode;
mod qclass;
mod qtype;
mod rcode;
mod rrclass;
mod rrtype;

pub use opcode::*;
pub use qclass::*;
pub use qtype::*;
pub use rcode::*;
pub use rrclass::*;
pub use rrtype::*;

/// DomainName max length.
///
/// [RFC 1035 ~3.1](https://tools.ietf.org/html/rfc1035#section-3.1)
pub const DOMAIN_NAME_MAX_LENGTH: usize = 255;

/// CharacterString max length.
///
/// [RFC 1035 ~3.3](https://tools.ietf.org/html/rfc1035#section-3.3)
pub const CHARACTER_STRING_MAX_LENGTH: usize = 255;

/// DomainName label max length.
///
/// [RFC 1035 ~3.1](https://tools.ietf.org/html/rfc1035#section-3.1)
pub const DOMAIN_NAME_LABEL_MAX_LENGTH: usize = 63;

/// DNS message header length.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035#section-4.1.1)
pub const HEADER_LENGTH: usize = 12;

/// Maximal length of a DNS message.
///
/// This value corresponds to the maximal possible length of a DNS
/// message when retrieved over TCP.
///
/// Defined in [RFC 1035 ~4.2.2](https://tools.ietf.org/html/rfc1035#section-4.2.2).
pub const DNS_MESSAGE_MAX_LENGTH: usize = u16::MAX as usize;
