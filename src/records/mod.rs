//! Resource records.

pub mod data;

mod opt;
pub use opt::*;

mod record;
pub use record::*;

mod record_set;
pub use record_set::*;

mod class;
pub use class::*;

mod r#type;
pub use r#type::*;
