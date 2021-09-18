//! Message reading primitives.

mod message_reader;
pub use message_reader::*;

mod questions;
pub use questions::*;

mod labels;
pub use labels::*;

mod name_ref;
pub use name_ref::*;

mod records;
pub use records::*;

mod section_tracker;
pub(crate) use section_tracker::*;

/// Offset in an encoded DNS message.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Offset(pub(crate) usize);
