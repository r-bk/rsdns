//! [rsdns](crate) is a DNS Client library providing functionality of a Stub Resolver defined in
//! [RFC 1034](https://www.rfc-editor.org/rfc/rfc1034.html#section-5.3.1).

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
pub(crate) mod macros;
pub(crate) mod bytes;
pub mod constants;
mod domain_name;
pub use domain_name::*;
pub mod errors;
pub mod message;
pub mod records;

cfg_any_resolver! {
    pub mod resolvers;
}

pub(crate) use errors::ProtocolResult;

#[doc(inline)]
pub use errors::{Error, Result};
