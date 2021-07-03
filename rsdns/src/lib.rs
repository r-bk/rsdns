#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! [rsdns](crate) is a DNS Client library providing functionality of a Stub Resolver defined in
//! [RFC 1034](https://tools.ietf.org/html/rfc1034#section-5.3.1).

pub(crate) mod bytes;
pub mod constants;
mod domain_name;
pub use domain_name::*;
mod error;
pub mod message;
pub mod records;

#[cfg(any(
    feature = "net-async-std",
    feature = "net-smol",
    feature = "net-std",
    feature = "net-tokio"
))]
pub mod resolvers;

pub(crate) use error::ProtocolResult;
pub use error::Result;
pub use error::{AnswerError, Error, ProtocolError};
