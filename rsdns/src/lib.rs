#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! [rsdns](crate) is a DNS Client library providing functionality of a Stub Resolver defined in
//! [RFC 1034](https://tools.ietf.org/html/rfc1034#section-5.3.1).

pub(crate) mod bytes;
mod character_string;
pub mod constants;
mod domain_name;
mod error;
mod flags;
mod header;
pub mod message;
#[cfg(any(
    feature = "net-async-std",
    feature = "net-smol",
    feature = "net-std",
    feature = "net-tokio"
))]
pub mod net;
mod question;

pub use character_string::*;
pub use domain_name::*;
pub use error::Error;
pub use error::Result;
pub use flags::*;
pub use header::*;
pub use question::*;
