//! Constants and definitions.

mod opcode;
pub use opcode::*;

mod class;
pub use class::*;

mod r#type;
pub use r#type::*;

mod rcode;
pub use rcode::*;

mod records_section;
pub use records_section::*;

// ----------------------------------------------------------------------------

/// Domain name max length.
///
/// [RFC 1035 section 3.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.1)
pub const DOMAIN_NAME_MAX_LENGTH: usize = 255;

/// Domain name label max length.
///
/// [RFC 1035 section 3.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.1)
pub const DOMAIN_NAME_LABEL_MAX_LENGTH: usize = 63;

/// Maximal number of pointers allowed in a single domain name.
///
/// This is [rsdns](crate)-specific constant.
pub const DOMAIN_NAME_MAX_POINTERS: usize = 32;

/// Message header length.
///
/// [RFC 1035 section 4.1.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.1)
pub const HEADER_LENGTH: usize = 12;

/// Maximal length of a message.
///
/// This value corresponds to the maximal possible length of a
/// message when retrieved over TCP.
///
/// [RFC 1035 section 4.2.2](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.2.2)
pub const DNS_MESSAGE_MAX_LENGTH: usize = u16::MAX as usize;
