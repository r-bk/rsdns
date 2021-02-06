//! DNS protocol implementation.
#[macro_use]
mod macros;
mod constants;
mod flags;
mod header;
pub use constants::*;
pub use flags::*;
pub use header::*;
