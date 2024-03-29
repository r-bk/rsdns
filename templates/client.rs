use crate::{
    clients::{
        {{ crate_module_name }}::ClientImpl,
        config::ClientConfig,
    },
    records::{data::RData, Class, RecordSet, Type},
    Result
};

{% if async == "true" -%}
{% set as = "async" %}
{% set aw = ".await" %}
/// Asynchronous client for the [`{{ crate_name }}`] async runtime.
///
/// [`{{ crate_name }}`]: https://docs.rs/{{ crate_name }}
{% else -%}
{% set as = "" %}
{% set aw = "" %}
/// Synchronous client implemented with [`std::net`].
///
/// [`std::net`]: https://doc.rust-lang.org/std/net
{% endif -%}
pub struct Client {
    internal: ClientImpl,
}

impl Client {
    /// Creates a new instance of [`Client`] with specified configuration.
    #[inline(always)]
    pub {{ as }} fn new(conf: ClientConfig) -> Result<Self> {
        conf.check()?;
        Ok(Self {
            internal: ClientImpl::new(conf){{ aw }}?,
        })
    }

    /// Returns the client configuration.
    #[inline(always)]
    pub fn config(&self) -> &ClientConfig {
        self.internal.config()
    }

    /// Issues a DNS query and writes the response into caller-owned buffer.
    ///
    /// This method gives the control over buffer management to the caller.
    /// The response message is written into `buf` and its length is returned in the result.
    ///
    /// See the [`message::reader`] module for ways to parse the received response.
    ///
    /// This method doesn't allocate.
    ///
    /// # Buffer size and EDNS
    ///
    /// The minimum size of `buf` is 512 bytes.
    /// When EDNS is enabled, the UDP payload size sent in the `OPT` record is the minimum between
    /// `udp_payload_size` configured in [`ClientConfig::edns`] and the size of `buf`.
    ///
    /// [`message::reader`]: crate::message::reader
    #[inline(always)]
    pub {{ as }} fn query_raw(&mut self, qname: &str, qtype: Type, qclass: Class, buf: &mut [u8]) -> Result<usize> {
        self.internal.query_raw(qname, qtype, qclass, buf){{ aw }}
    }

    /// Issues a DNS query and returns the resulting [`RecordSet`].
    ///
    /// Usually the resulting record set will belong to the domain name specified in `qname`.
    /// However, if `qname` has a [`CNAME`] record, the record set will belong to `qname`'s
    /// canonical name. See [`RecordSet::from_msg`] for *CNAME flattening* description.
    ///
    /// This method allows data-type queries only.
    /// For meta-queries (e.g. [`Type::ANY`]) use [`query_raw`].
    ///
    /// This method allocates.
    ///
    /// [`CNAME`]: crate::records::data::Cname
    /// [`query_raw`]: Self::query_raw
    pub {{ as }} fn query_rrset<D: RData>(&mut self, qname: &str, qclass: Class) -> Result<RecordSet<D>> {
        self.internal.query_rrset(qname, qclass){{ aw }}
    }
}
