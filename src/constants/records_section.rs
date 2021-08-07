use std::fmt::{self, Display, Formatter};

/// Message sections conveying resource records.
///
/// A DNS message is divided into sections of different types.
/// These are the sections conveying resource records.
///
/// [RFC 1035 section 4.1.3](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.3)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub enum RecordsSection {
    /// The answer section.
    Answer = 0,
    /// The authority section.
    Authority = 1,
    /// The additional section.
    Additional = 2,
}

impl RecordsSection {
    /// Converts `RecordsSection` to a static string.
    pub fn to_str(self) -> &'static str {
        match self {
            RecordsSection::Answer => "Answer",
            RecordsSection::Authority => "Authority",
            RecordsSection::Additional => "Additional",
        }
    }
}

impl Display for RecordsSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.to_str())
    }
}
