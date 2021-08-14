/// Protocol selection strategy.
///
/// Defines resolver's usage of UDP and TCP protocols.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ProtocolStrategy {
    /// Use UDP by default. Fallback to TCP on truncated responses.
    ///
    /// The default strategy is to use UDP for all queries except for
    /// [`Type::Any`](crate::constants::Type::Any).
    /// If a UDP response is truncated, TCP is used to issue another query and receive a full
    /// response.
    ///
    /// Queries that by definition are required to use only TCP are exempt from these rules.
    Default,

    /// Use UDP for all queries including [`Type::Any`](crate::constants::Type::Any).
    ///
    /// By default [`Type::Any`](crate::constants::Type::Any) queries use TCP only.
    /// This setting will force UDP to be used for these queries too.
    ///
    /// Queries that by definition are required to use only TCP are exempt from these rules.
    Udp,

    /// Use TCP for all queries.
    Tcp,

    /// Do not use TCP.
    ///
    /// This setting forces usage of UDP only. Truncated responses are returned as is - without
    /// being retried.
    ///
    /// Queries that by definition are required to use only TCP are exempt from these rules.
    NoTcp,
}
