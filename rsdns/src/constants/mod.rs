//! Constants and definitions.

mod opcode;
mod qclass;
mod qtype;
mod rclass;
mod response_code;
mod rtype;
mod section;

pub use opcode::*;
pub use qclass::*;
pub use qtype::*;
pub use rclass::*;
pub use response_code::*;
pub use rtype::*;
pub use section::*;

/// DomainName max length.
///
/// [RFC 1035 ~3.1](https://tools.ietf.org/html/rfc1035#section-3.1)
pub const DOMAIN_NAME_MAX_LENGTH: usize = 255;

/// DomainName label max length.
///
/// [RFC 1035 ~3.1](https://tools.ietf.org/html/rfc1035#section-3.1)
pub const DOMAIN_NAME_LABEL_MAX_LENGTH: usize = 63;

/// Message header length.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035#section-4.1.1)
pub const HEADER_LENGTH: usize = 12;

/// Maximal length of a message.
///
/// This value corresponds to the maximal possible length of a
/// message when retrieved over TCP.
///
/// Defined in [RFC 1035 ~4.2.2](https://tools.ietf.org/html/rfc1035#section-4.2.2).
pub const DNS_MESSAGE_MAX_LENGTH: usize = u16::MAX as usize;
