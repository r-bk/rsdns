//! Resolver configuration.

mod protocol_strategy;
pub use protocol_strategy::*;

mod recursion;
pub use recursion::*;

mod resolver_config;
pub use resolver_config::*;

#[cfg(windows)]
pub(crate) mod win;

#[cfg(unix)]
pub(crate) mod uni;
