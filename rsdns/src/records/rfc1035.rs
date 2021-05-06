use crate::constants::RType;

rr!(
    /// A host address (IPv4).
    ///
    /// [`RFC 1035 ~3.4.1`](https://tools.ietf.org/html/rfc1035#section-3.4.1)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    A,
    RType::A
);

rr!(
    /// A well known service description.
    ///
    /// [`RFC 1035 ~3.4.2`](https://tools.ietf.org/html/rfc1035#section-3.4.2)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Wks,
    RType::WKS
);

rr!(
    /// The canonical name for an alias.
    ///
    /// [`RFC 1035 ~3.3.1`](https://tools.ietf.org/html/rfc1035#section-3.3.1)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Cname,
    RType::CNAME
);

rr!(
    /// Host information.
    ///
    /// Standard values for CPU and OS can be found in
    /// [`RFC 1010`](https://tools.ietf.org/html/rfc1010).
    ///
    /// [`RFC 1035 ~3.3.2`](https://tools.ietf.org/html/rfc1035#section-3.3.2)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Hinfo,
    RType::HINFO
);

rr!(
    /// A mailbox domain name.
    ///
    /// [`RFC 1035 ~3.3.3`](https://tools.ietf.org/html/rfc1035#section-3.3.3)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Mb,
    RType::MB
);

rr!(
    /// A mail destination.
    ///
    /// Obsolete.
    ///
    /// [`RFC 1035 ~3.3.4`](https://tools.ietf.org/html/rfc1035#section-3.3.4)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Md,
    RType::MD
);

rr!(
    /// A mail forwarder.
    ///
    /// Obsolete.
    ///
    /// [`RFC 1035 ~3.3.5`](https://tools.ietf.org/html/rfc1035#section-3.3.5)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Mf,
    RType::MF
);

rr!(
    /// A mail group member.
    ///
    /// [`RFC 1035 ~3.3.6`](https://tools.ietf.org/html/rfc1035#section-3.3.6)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Mg,
    RType::MG
);

rr!(
    /// Mailbox or mail list information.
    ///
    /// [`RFC 1035 ~3.3.7`](https://tools.ietf.org/html/rfc1035#section-3.3.7)
    #[derive(Clone, Eq, PartialEq, Hash, Debug)]
    Minfo,
    RType::MINFO
);

rr!(
    /// A mail rename domain name.
    ///
    /// [`RFC 1035 ~3.3.8`](https://tools.ietf.org/html/rfc1035#section-3.3.8)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Mr,
    RType::MR
);

rr!(
    /// Mail exchange.
    ///
    /// [`RFC 1035 ~3.3.9`](https://tools.ietf.org/html/rfc1035#section-3.3.9)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Mx,
    RType::MX
);

rr!(
    /// The Null record.
    ///
    /// [`RFC 1035 ~3.3.10`](https://tools.ietf.org/html/rfc1035#section-3.3.10)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Null,
    RType::NULL
);

rr!(
    /// An authoritative name server.
    ///
    /// [`RFC 1035 ~3.3.11`](https://tools.ietf.org/html/rfc1035#section-3.3.11)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Ns,
    RType::NS
);

rr!(
    /// A domain name pointer.
    ///
    /// [`RFC 1035 ~3.3.12`](https://tools.ietf.org/html/rfc1035#section-3.3.12)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Ptr,
    RType::PTR
);

rr!(
    /// Marks the start of a zone of authority.
    ///
    /// [`RFC 1035 ~3.3.13`](https://tools.ietf.org/html/rfc1035#section-3.3.13)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Soa,
    RType::SOA
);

rr!(
    /// Text strings.
    ///
    /// [`RFC 1035 ~3.3.14`](https://tools.ietf.org/html/rfc1035#section-3.3.14)
    #[derive(Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
    Txt,
    RType::TXT
);
