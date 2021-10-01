/// EDNS configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EDns {
    /// EDNS is disabled.
    Off,

    /// EDNS is enabled.
    On {
        /// The EDNS version.
        ///
        /// Default: `0`
        version: u8,

        /// Specifies the max size (in bytes) of UDP payload the client is capable of
        /// receiving. This value allows DNS query responses longer than 512 bytes.
        /// The buffer used for DNS message reception must be equal or longer than this value.
        udp_payload_size: u16,
    },
}
