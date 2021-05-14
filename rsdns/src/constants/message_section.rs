/// Message section.
#[derive(Copy, Clone, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub enum MessageSection {
    /// The header section.
    ///
    /// [`RFC 1035 ~4.1.1`](https://tools.ietf.org/html/rfc1035#section-4.1.1)
    Header = 0,
    /// The question section.
    ///
    /// [`RFC 1035 ~4.1.2`](https://tools.ietf.org/html/rfc1035#section-4.1.2)
    Question = 1,
    /// The answer section.
    ///
    /// [`RFC 1035 ~4.1.3`](https://tools.ietf.org/html/rfc1035#section-4.1.3)
    Answer = 2,
    /// The authority section.
    ///
    /// [`RFC 1035 ~4.1.3`](https://tools.ietf.org/html/rfc1035#section-4.1.3)
    Authority = 3,
    /// The additional section.
    ///
    /// [`RFC 1035 ~4.1.3`](https://tools.ietf.org/html/rfc1035#section-4.1.3)
    Additional = 4,
}
