use bitflags::bitflags;
use std::fmt::{self, Display, Formatter};

bitflags! {
    /// Message sections conveying resource records (as bitflags).
    ///
    /// A DNS message is divided into sections of different types.
    /// These are the sections conveying resource records.
    ///
    /// The bitflags are used where multiple sections may be specified,
    /// which cannot be done with the [`RecordsSection`] enum.
    ///
    /// [RFC 1035 section 4.1.3](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.3)
    ///
    /// [`RecordsSection`]: super::RecordsSection
    pub struct RecordsSections: u16 {
        /// The answer section.
        const ANSWER     = 0b0001;
        /// The authority section.
        const AUTHORITY  = 0b0010;
        /// The additional section.
        const ADDITIONAL = 0b0100;
    }
}

impl RecordsSections {
    /// Converts `RecordsSections` to a static string.
    pub fn to_str(self) -> &'static str {
        if self == Self::ANSWER {
            "ANSWER"
        } else if self == Self::AUTHORITY {
            "AUTHORITY"
        } else if self == Self::ADDITIONAL {
            "ADDITIONAL"
        } else if self == Self::ANSWER.union(Self::AUTHORITY) {
            "ANSWER | AUTHORITY"
        } else if self == Self::ANSWER.union(Self::ADDITIONAL) {
            "ANSWER | ADDITIONAL"
        } else if self == Self::AUTHORITY.union(Self::ADDITIONAL) {
            "AUTHORITY | ADDITIONAL"
        } else if self == Self::all() {
            "ANSWER | AUTHORITY | ADDITIONAL"
        } else {
            "__NOT_A_RECORDS_SECTION__"
        }
    }
}

impl Display for RecordsSections {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.to_str())
    }
}
