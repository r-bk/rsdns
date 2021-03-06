//! Defines configuration for resolvers.
use crate::resolvers::config::{ProtocolStrategy, Recursion};
use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    time::Duration,
};

#[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
use crate::{Error, Result};

#[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
const INTERFACE_NAME_MAX_LENGTH: usize = 16; // socket(7), IFNAMSIZ

#[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
type InterfaceName = arrayvec::ArrayString<INTERFACE_NAME_MAX_LENGTH>;

/// Configuration for resolvers.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ResolverConfig {
    pub(crate) nameserver_: SocketAddr,
    pub(crate) bind_addr_: SocketAddr,
    #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
    pub(crate) interface_: InterfaceName,
    pub(crate) query_lifetime_: Duration,
    pub(crate) query_timeout_: Option<Duration>,
    pub(crate) protocol_strategy_: ProtocolStrategy,
    pub(crate) recursion_: Recursion,
}

impl ResolverConfig {
    /// Creates resolver configuration for `nameserver` with default values.
    pub fn new(nameserver: SocketAddr) -> ResolverConfig {
        let bind_addr = if nameserver.is_ipv4() {
            Self::default_ipv4_bind_address()
        } else {
            Self::default_ipv6_bind_address()
        };
        ResolverConfig {
            nameserver_: nameserver,
            bind_addr_: bind_addr,
            #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
            interface_: InterfaceName::default(),
            query_lifetime_: Duration::from_millis(10000),
            query_timeout_: Some(Duration::from_millis(2000)),
            protocol_strategy_: ProtocolStrategy::Default,
            recursion_: Recursion::On,
        }
    }

    /// Returns the nameserver address.
    pub fn nameserver(&self) -> SocketAddr {
        self.nameserver_
    }

    /// Sets the nameserver address.
    ///
    /// This method is useful to create identical configuration for several nameservers.
    ///
    /// # Examples
    /// ```rust
    /// # use rsdns::resolvers::config::ResolverConfig;
    /// # use std::{net::SocketAddr, str::FromStr, time::Duration};
    /// # fn foo() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// let conf1 = ResolverConfig::new(SocketAddr::from_str("127.0.0.53:53")?)
    ///     .set_query_lifetime(Duration::from_secs(5));
    ///
    /// let conf2 = conf1
    ///     .clone()
    ///     .set_nameserver(SocketAddr::from_str("8.8.8.8:53")?);
    ///
    /// assert_eq!(conf1.nameserver(), SocketAddr::from_str("127.0.0.53:53")?);
    /// assert_eq!(conf1.query_lifetime(), Duration::from_secs(5));
    /// assert_eq!(conf2.nameserver(), SocketAddr::from_str("8.8.8.8:53")?);
    /// assert_eq!(conf2.query_lifetime(), Duration::from_secs(5));
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_nameserver(mut self, nameserver: SocketAddr) -> Self {
        self.nameserver_ = nameserver;

        if self.nameserver_.is_ipv6() && self.bind_addr_ == Self::default_ipv4_bind_address() {
            self.bind_addr_ = Self::default_ipv6_bind_address();
        }

        if self.nameserver_.is_ipv4() && self.bind_addr_ == Self::default_ipv6_bind_address() {
            self.bind_addr_ = Self::default_ipv4_bind_address();
        }

        self
    }

    /// Returns the socket local bind address.
    ///
    /// Default: `0.0.0.0:0`.
    pub fn bind_addr(&self) -> SocketAddr {
        self.bind_addr_
    }

    /// Sets the socket local bind address.
    ///
    /// Defines the address UDP sockets are bound to.
    ///
    /// Default: `0.0.0.0:0`.
    pub fn set_bind_addr(mut self, bind_addr: SocketAddr) -> Self {
        self.bind_addr_ = bind_addr;
        self
    }

    /// Returns the interface name to bind to.
    ///
    /// Default: `None`.
    #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2")))
    )]
    pub fn bind_device(&self) -> Option<&str> {
        if !self.interface_.is_empty() {
            Some(self.interface_.as_str())
        } else {
            None
        }
    }

    /// Sets the interface name to bind to.
    ///
    /// This option forces the resolver to bind sockets to a specified interface using the
    /// `SO_BINDTODEVICE` socket option (see `socket(7)` man page).
    ///
    /// `interface_name` should be a non-empty string shorter than 16 bytes (`IFNAMSIZ`).
    /// Whitespace characters and `'/'` are considered invalid for interface names.
    ///
    /// This option is handy when you have multiple network interfaces with the same IP address.
    /// In this case [Self::bind_addr] cannot be used to identify the correct network interface.
    ///
    /// Default: `None`.
    #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2")))
    )]
    pub fn set_bind_device(mut self, interface_name: Option<&str>) -> Result<Self> {
        match interface_name {
            Some(bd) => {
                if bd.is_empty() || bd.len() >= self.interface_.capacity() {
                    return Err(Error::BadInput("invalid interface name length"));
                }
                for b in bd.as_bytes() {
                    if b.is_ascii_whitespace() || *b == b'/' {
                        return Err(Error::BadInput(
                            "interface name contains forbidden characters - '/' or whitespace",
                        ));
                    }
                }
                self.interface_.clear();
                self.interface_.try_push_str(bd).ok();
            }
            None => self.interface_.clear(),
        }
        Ok(self)
    }

    /// Returns the query lifetime duration.
    ///
    /// Default: `10 sec`.
    pub fn query_lifetime(&self) -> Duration {
        self.query_lifetime_
    }

    /// Sets the query lifetime duration.
    ///
    /// Query lifetime duration is the upper bound on the overall query duration, including all
    /// UDP retries. This value should be greater than [Self::query_timeout] if the latter
    /// is set.
    ///
    /// Default: `10 sec`.
    pub fn set_query_lifetime(mut self, query_lifetime: Duration) -> Self {
        self.query_lifetime_ = query_lifetime;
        self
    }

    /// Returns the UDP query timeout duration.
    ///
    /// Default: `2 sec`.
    pub fn query_timeout(&self) -> Option<Duration> {
        self.query_timeout_
    }

    /// Sets the UDP query timeout duration.
    ///
    /// Denotes the time to resend an unanswered UDP query. This value should be smaller than
    /// [Self::query_lifetime]. During query lifetime it may be resent several times before
    /// timing out.
    ///
    /// This option may be `None`, in which case an unanswered UDP query is never resent.
    ///
    /// Default: `2 sec`.
    pub fn set_query_timeout(mut self, query_timeout: Option<Duration>) -> Self {
        self.query_timeout_ = query_timeout;
        self
    }

    /// Returns the protocol strategy.
    pub fn protocol_strategy(&self) -> ProtocolStrategy {
        self.protocol_strategy_
    }

    /// Sets the protocol strategy.
    ///
    /// See [`ProtocolStrategy`] for more info.
    pub fn set_protocol_strategy(mut self, strategy: ProtocolStrategy) -> Self {
        self.protocol_strategy_ = strategy;
        self
    }

    /// Returns the recursion option.
    ///
    /// Default: `Recursion::On`.
    pub fn recursion(&self) -> Recursion {
        self.recursion_
    }

    /// Sets the recursion option.
    ///
    /// Specifies if to set the recursion flag in the query.
    ///
    /// Default: `Recursion::On`.
    pub fn set_recursion(mut self, recursion: Recursion) -> Self {
        self.recursion_ = recursion;
        self
    }

    fn default_ipv4_bind_address() -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0))
    }

    fn default_ipv6_bind_address() -> SocketAddr {
        SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0))
    }
}
