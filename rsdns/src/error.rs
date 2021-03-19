/// Errors returned by this library.
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown RR TYPE: {0}")]
    UnknownRrType(u16),
    #[error("unknown QTYPE: {0}")]
    UnknownQType(u16),
    #[error("unknown RR CLASS: {0}")]
    UnknownRrClass(u16),
    #[error("unknown QCLASS: {0}")]
    UnknownQClass(u16),
    #[error("unknown OPCODE: {0}")]
    UnknownOpCode(u8),
    #[error("unknown RCODE: {0}")]
    UnknownRCode(u8),
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("domain name label invalid character")]
    DomainNameLabelInvalidChar,
    #[error("domain name label is malformed")]
    DomainNameLabelMalformed,
    #[error("domain name label length exceeds allowed limit 63: {0}")]
    DomainNameLabelTooLong(usize),
    #[error("domain name length exceeds allowed limit")]
    DomainNameTooLong,
    #[error("domain name pointer loop detected")]
    DomainNamePointerLoop,
    #[error("domain name pointer count exceeds allowed limit")]
    DomainNameTooMuchPointers,
    #[error("domain name label type is invalid")]
    DomainNameBadLabelType,
    #[error("domain name label pointer is invalid")]
    DomainNameBadPointer,
    #[error("buffer end unexpectedly reached")]
    EndOfBuffer,
}

/// Result returned by this library.
pub type Result<T> = std::result::Result<T, Error>;
