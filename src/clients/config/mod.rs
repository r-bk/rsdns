//! Client configuration.

mod edns;
pub use edns::*;

mod protocol_strategy;
pub use protocol_strategy::*;

mod recursion;
pub use recursion::*;

mod client_config;
pub use client_config::*;
