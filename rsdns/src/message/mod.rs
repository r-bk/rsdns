//! Message handling.

mod character_string;
mod flags;
mod header;
mod message_type;
mod operation_code;
#[cfg(disabled)]
mod query_class;
#[cfg(disabled)]
mod query_type;
mod query_writer;
mod question;
pub mod reader;
#[cfg(disabled)]
mod record_class;
#[cfg(disabled)]
mod record_type;
mod response_code;

pub use flags::*;
pub use header::*;
pub use message_type::*;
pub use operation_code::*;
#[cfg(disabled)]
pub use query_class::*;
#[cfg(disabled)]
pub use query_type::*;
#[allow(unused_imports)]
pub(crate) use query_writer::*;
pub use question::*;
#[cfg(disabled)]
pub use record_class::*;
#[cfg(disabled)]
pub use record_type::*;
pub use response_code::*;
