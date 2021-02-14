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

/// DomainName label max length.
///
/// [RFC 1035 ~3.1](https://tools.ietf.org/html/rfc1035#section-3.1)
pub const DOMAIN_NAME_LABEL_MAX_LENGTH: usize = 63;
