/// Message section.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub enum MessageSection {
    /// The header section.
    Header = 0,
    /// The question section.
    Question = 1,
    /// The answert section.
    Answer = 2,
    /// The authority section.
    Authority = 3,
    /// The additional section.
    Additional = 4,
}
