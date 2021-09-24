//! Message reading primitives.
//!
//! This module is dedicated to efficient parsing of DNS messages.
//! For more details see [`MessageReader`], which is the main struct to be used for this purpose.

mod message_reader;
pub use message_reader::*;

mod questions;
pub use questions::*;

mod labels;
pub use labels::*;

mod name_ref;
pub use name_ref::*;

mod question_ref;
pub use question_ref::*;

mod records;
pub use records::*;

mod records_reader;
pub use records_reader::*;

mod section_tracker;
pub(crate) use section_tracker::*;
