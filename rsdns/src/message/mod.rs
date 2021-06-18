//! Message handling.

mod character_string;
mod flags;
mod header;
mod message_type;
mod operation_code;
mod query_type;
mod query_writer;
mod question;
pub mod reader;
mod record_type;
mod response_code;

pub use flags::*;
pub use header::*;
pub use message_type::*;
pub use operation_code::*;
pub use query_type::*;
#[allow(unused_imports)]
pub(crate) use query_writer::*;
pub use question::*;
pub use record_type::*;
pub use response_code::*;
