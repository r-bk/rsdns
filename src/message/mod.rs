//! Message handling.

mod character_string;

mod flags;
pub use flags::*;

mod header;
pub use header::*;

mod message_type;
pub use message_type::*;

mod records_section;
pub use records_section::*;

mod opcode;
pub use opcode::*;

cfg_any_client! {
    mod query_writer;

    #[cfg_attr(test, allow(unused_imports))]
    pub(crate) use query_writer::*;
}

mod question;
pub use question::*;

pub mod reader;

mod rcode;
pub use rcode::*;
