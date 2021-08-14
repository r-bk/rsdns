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

cfg_any_resolver! {
    mod query_writer;

    #[cfg_attr(test, allow(unused_imports))]
    pub(crate) use query_writer::*;
}

mod question;
pub use question::*;

pub mod reader;

mod rcode_value;
pub use rcode_value::*;

mod class_value;
pub use class_value::*;

mod type_value;
pub use type_value::*;
