//! Defines entities for parsing a raw DNS message.

mod message_reader;
mod questions;

pub use message_reader::*;
pub use questions::*;
