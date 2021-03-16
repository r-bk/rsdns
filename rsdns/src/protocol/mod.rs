//! DNS protocol implementation.
#[macro_use]
mod macros;
pub(crate) mod bytes;
mod constants;
mod domain_name;
mod flags;
mod header;
pub mod message;
mod question;
mod resource_record;

pub use constants::*;
pub use domain_name::*;
pub use flags::*;
pub use header::*;
pub use question::*;
pub use resource_record::*;
