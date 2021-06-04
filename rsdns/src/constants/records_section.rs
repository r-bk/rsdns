/// Message sections conveying resource records.
///
/// A DNS message is divided into sections. Part of them convey resource records.
///
/// [`RFC 1035 ~4.1.3`](https://tools.ietf.org/html/rfc1035#section-4.1.3)
#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub enum RecordsSection {
    /// The answer section.
    Answer = 0,
    /// The authority section.
    Authority = 1,
    /// The additional section.
    Additional = 2,
}
