//! Error types.

use crate::{
    constants::{
        RClass, RType, DOMAIN_NAME_LABEL_MAX_LENGTH, DOMAIN_NAME_MAX_LENGTH,
        DOMAIN_NAME_MAX_POINTERS,
    },
    message::{MessageType, OperationCode, RecordClass, RecordType, ResponseCode},
};

/// Errors returned by [rsdns](crate).
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// IoError as returned from the underlying IO engine
    #[error("io error")]
    IoError(#[from] std::io::Error),

    #[error("unknown record type: {0}")]
    UnknownRecordType(RecordType),

    #[error("RTYPE {0} is not expected")]
    UnexpectedRType(RType),

    #[error("unknown record class: {0}")]
    UnknownRecordClass(RecordClass),

    #[error("unknown operation code: {0}")]
    UnknownOperationCode(OperationCode),

    #[error("unknown response code: {0}")]
    UnknownResponseCode(ResponseCode),

    #[error("{0}: {1:#02X}")]
    DomainNameLabelInvalidChar(&'static str, u8),

    #[error(
        "domain name label length exceeds allowed limit {}: {0}",
        DOMAIN_NAME_LABEL_MAX_LENGTH
    )]
    DomainNameLabelTooLong(usize),

    #[error("domain name label is empty")]
    DomainNameLabelIsEmpty,

    #[error(
        "domain name length exceeds allowed limit {}: {0}",
        DOMAIN_NAME_MAX_LENGTH
    )]
    DomainNameTooLong(usize),

    #[error("domain name pointer loop detected from offset {src} to offset {dst}")]
    DomainNamePointerLoop { src: usize, dst: usize },

    #[error(
        "domain name pointer count exceeds allowed limit {}",
        DOMAIN_NAME_MAX_POINTERS
    )]
    DomainNameTooMuchPointers,

    #[error("domain name label type is invalid: label = {0:#02X}")]
    DomainNameBadLabelType(u8),

    #[error("domain name label pointer {pointer} exceeds buffer max offset {max_offset}")]
    DomainNameBadPointer { pointer: usize, max_offset: usize },

    #[error("buffer end reached unexpectedly")]
    EndOfBuffer,

    #[error("buffer window end reached unexpectedly")]
    EndOfWindow,

    #[error("cursor is already in window mode")]
    CursorAlreadyInWindow,

    #[error("cursor not in window mode")]
    CursorNotInWindow,

    #[error("cursor window error: window_end {window_end}, pos {pos}")]
    CursorWindowError { window_end: usize, pos: usize },

    #[error("buffer is not large enough: {0} bytes required")]
    BufferTooShort(usize),

    #[error("malformed message: question is missing")]
    MessageWithoutQuestion,

    #[error("message type {0} is incompatible in this context")]
    BadMessageType(MessageType),

    #[error("bad response code: {0}")]
    BadResponseCode(ResponseCode),

    #[error("message is truncated")]
    MessageTruncated,

    #[error("message contains no records that answer the query")]
    NoAnswer,

    /// Resolver API is supported for a subset of record types
    #[error("RType {0} is not supported")]
    UnsupportedRType(RType),

    /// Resolver API is supported for a subset of record classes
    #[error("RClass {0} is not supported")]
    UnsupportedRClass(RClass),

    /// Generic timeout error
    #[error("operation timed-out")]
    Timeout,

    /// Generic bad input error
    #[error("bad input: {0}")]
    BadInput(&'static str),

    /// *rsdns* tries to avoid panics. InternalError is used instead.
    #[error("internal error: {0}")]
    InternalError(&'static str),
}

/// Result returned by [rsdns](crate).
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_size() {
        println!("size_of(Error) = {}", std::mem::size_of::<Error>());
        println!(
            "size_of(io::Error) = {}",
            std::mem::size_of::<std::io::Error>()
        );

        assert_eq!(std::mem::size_of::<Error>(), 24);
    }
}
