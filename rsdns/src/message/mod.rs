//! Message handling.

mod character_string;

mod flags;
pub use flags::*;

mod header;
pub use header::*;

mod message_type;
pub use message_type::*;

mod operation_code;
pub use operation_code::*;

mod query_writer;
pub(crate) use query_writer::*;

mod question;
pub use question::*;

pub mod reader;

mod response_code;
pub use response_code::*;
