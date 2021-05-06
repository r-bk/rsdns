use crate::constants::RType;

rr!(
    /// A host address (IPv6).
    ///
    /// [`RFC 3596 ~2.2`](https://tools.ietf.org/html/rfc3596#section-2.2)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Aaaa,
    RType::AAAA
);
