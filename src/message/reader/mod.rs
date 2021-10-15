//! Message reading primitives.
//!
//! There are two main primitives to read a DNS message:
//! 1. [`MessageReader`] has a wide variety of methods that can be tailored to a very efficient
//!    message decoding.
//! 2. [`MessageIterator`] is made for convenience.
//!    It allows parsing a message in way of a Rust `Iterator`.

mod message_iterator;
pub use message_iterator::*;

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

mod message_reader;
pub use message_reader::*;

mod section_tracker;
pub(crate) use section_tracker::*;
