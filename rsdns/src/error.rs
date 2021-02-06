use thiserror::Error;

/// Errors returned by this library.
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum RsDnsError {
    #[error("protocol: unknown TYPE: {0}")]
    ProtocolUnknownRrType(u16),
    #[error("protocol: unknown QTYPE: {0}")]
    ProtocolUnknownQType(u16),
    #[error("protocol: unknown CLASS: {0}")]
    ProtocolUnknownRrClass(u16),
    #[error("protocol: unknown QCLASS: {0}")]
    ProtocolUnknownQClass(u16),
    #[error("protocol: unknown OPCODE: {0}")]
    ProtocolUnknownOpCode(u8),
    #[error("protocol: unknown RCODE: {0}")]
    ProtocolUnknownRCode(u8),
    #[error("io error")]
    IoError(#[from] std::io::Error),
}

/// Result returned by this library.
pub type Result<T> = std::result::Result<T, RsDnsError>;
