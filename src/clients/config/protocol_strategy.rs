/// Protocol selection strategy.
///
/// Defines client's usage of UDP and TCP protocols.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ProtocolStrategy {
    /// Use UDP by default. Fallback to TCP on truncated responses.
    ///
    /// Queries that by definition are required to use only TCP are exempt from these rules.
    Udp,

    /// Use only TCP for all queries.
    Tcp,

    /// Do not use TCP.
    ///
    /// This setting forces usage of UDP only. Truncated responses are returned as is, without
    /// being retried.
    ///
    /// Queries that by definition are required to use only TCP are exempt from these rules.
    NoTcp,
}
