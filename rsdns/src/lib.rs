#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! [rsdns](crate) is a DNS Client library providing functionality of a Stub Resolver defined in
//! [RFC 1034](https://tools.ietf.org/html/rfc1034#section-5.3.1).

pub(crate) mod bytes;
mod character_string;
pub mod constants;
mod error;
#[cfg(any(
    feature = "net-async-std",
    feature = "net-smol",
    feature = "net-std",
    feature = "net-tokio"
))]
pub mod net;
pub mod protocol;
mod question;

pub use character_string::*;
pub use error::Error;
pub use error::Result;
pub use question::*;
