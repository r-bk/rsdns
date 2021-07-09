//! Error types.

use crate::{constants::CNAME_CHAIN_MAX_LENGTH, message::ResponseCode};

/// Variants of [Error::ProtocolError].
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum ProtocolError {
    #[error("reserved resource record type: {0}")]
    ReservedRType(u16),
    #[error("reserved query type: {0}")]
    ReservedQType(u16),
    #[error("reserved resource record class: {0}")]
    ReservedRClass(u16),
    #[error("reserved query class: {0}")]
    ReservedQClass(u16),
    #[error("reserved query opcode: {0}")]
    ReservedOpCode(u8),
    #[error("reserved response code: {0}")]
    ReservedRCode(u16),
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
    #[error("buffer window end reached unexpectedly")]
    EndOfWindow,
    #[error("cursor is already in window mode")]
    CursorAlreadyInWindow,
    #[error("cursor not in window mode")]
    CursorNotInWindow,
    #[error("cursor window error: expected {0}, actual {1}")]
    CursorWindowError(usize, usize),
    #[error("buffer is not large enough: {0} bytes required")]
    BufferTooShort(usize),
    #[error("malformed message: no question")]
    NoQuestion,
}

/// Variants of [Error::AnswerError].
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum AnswerError {
    #[error("message type is not Response")]
    NotResponse,
    #[error("response message is truncated")]
    Truncated,
    #[error("message contains no records that answer the query")]
    NoAnswer,
    #[error("CNAME chain exceeds allowed limit: {}", CNAME_CHAIN_MAX_LENGTH)]
    CnameChainTooLong,
    #[error("bad response code: {0}")]
    BadResponseCode(ResponseCode),
}

/// Errors returned by [rsdns](crate).
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("protocol error: {0}")]
    ProtocolError(#[from] ProtocolError),
    #[error(transparent)]
    AnswerError(AnswerError),
    #[error("operation timed-out")]
    Timeout,
    #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2")))
    )]
    #[error("device name is too long or contains forbidden characters - '/' or whitespace")]
    BadBindDevice,
    #[error("iterator exhausted")]
    IterationStop,
    #[error("bad string")]
    BadStr,
    #[error("internal error: {0}")]
    InternalError(String),
}

/// Result returned by [rsdns](crate).
pub type Result<T> = std::result::Result<T, Error>;

/// Result returned by protocol-related functions.
pub(crate) type ProtocolResult<T> = std::result::Result<T, ProtocolError>;
