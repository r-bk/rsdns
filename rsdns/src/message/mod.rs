//! Message handling.

mod character_string;
mod flags;
mod header;
mod message_type;
mod parsed_opcode;
mod query_writer;
mod question;
pub mod reader;

pub use flags::*;
pub use header::*;
pub use message_type::*;
pub use parsed_opcode::*;
#[allow(unused_imports)]
pub(crate) use query_writer::*;
pub use question::*;
