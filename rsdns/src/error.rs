/// Errors returned by this library.
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown resource record type: {0}")]
    UnknownRType(u16),
    #[error("unknown QTYPE: {0}")]
    UnknownQType(u16),
    #[error("unknown resource record class: {0}")]
    UnknownRClass(u16),
    #[error("unknown QCLASS: {0}")]
    UnknownQClass(u16),
    #[error("unknown OPCODE: {0}")]
    UnknownOpCode(u8),
    #[error("unknown response code: {0}")]
    UnknownResponseCode(u8),
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
    #[error("buffer end reached unexpectedly")]
    EndOfBuffer,
    #[error("buffer is not large enough: {0} bytes required")]
    BufferTooShort(usize),
    #[error("operation timed-out")]
    Timeout,
    #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2")))
    )]
    #[error("device name is too long or contains forbidden characters - '/' or whitespace")]
    BadBindDevice,
}

/// Result returned by this library.
pub type Result<T> = std::result::Result<T, Error>;
