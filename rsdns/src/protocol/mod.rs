//! Protocol implementation.

#[macro_use]
mod macros;
mod character_string;
mod domain_name;
mod flags;
mod header;
pub mod message;
mod question;

pub use character_string::*;
pub use domain_name::*;
pub use flags::*;
pub use header::*;
pub use question::*;
