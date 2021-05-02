//! Protocol implementation.

#[macro_use]
mod macros;
pub(crate) mod bytes;
mod character_string;
mod constants;
mod domain_name;
mod flags;
mod header;
pub mod message;
mod question;

pub use character_string::*;
pub use constants::*;
pub use domain_name::*;
pub use flags::*;
pub use header::*;
pub use question::*;
