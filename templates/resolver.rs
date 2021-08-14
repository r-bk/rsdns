use crate::{
  constants::{Type, RClass},
  records::{data::RData, RecordSet},
  resolvers::{
      {{ crate_module_name }}::ResolverImpl,
      config::ResolverConfig,
  },
  Result
};

{% if async == "true" -%}
{% set as = "async" %}
{% set aw = ".await" %}
/// Asynchronous resolver for the [`{{ crate_name }}`] async runtime.
///
/// [`{{ crate_name }}`]: https://docs.rs/{{ crate_name }}
{% else -%}
{% set as = "" %}
{% set aw = "" %}
/// Synchronous resolver implemented on top of [`std::net`].
///
/// [`std::net`]: https://doc.rust-lang.org/std/net
{% endif -%}
pub struct Resolver {
    internal: ResolverImpl,
}

impl Resolver {
    /// Creates a new instance of [`Resolver`] with specified configuration.
    #[inline(always)]
    pub {{ as }} fn new(conf: ResolverConfig) -> Result<Self> {
        Ok(Self {
            internal: ResolverImpl::new(conf){{ aw }}?,
        })
    }

    /// Returns the resolver configuration.
    #[inline(always)]
    pub fn config(&self) -> &ResolverConfig {
        self.internal.config()
    }

    /// Issues a DNS query and writes the response into caller-owned buffer.
    ///
    /// This method gives the control over buffer management to the caller.
    /// The response message is written into `buf` and its length is returned in the result.
    ///
    /// The received response can be parsed using [`MessageReader`].
    ///
    /// This method doesn't allocate.
    ///
    /// [`MessageReader`]: crate::message::reader::MessageReader
    #[inline(always)]
    pub {{ as }} fn query_raw(&mut self, qname: &str, qtype: Type, qclass: RClass, buf: &mut [u8]) -> Result<usize> {
        self.internal.query_raw(qname, qtype, qclass, buf){{ aw }}
    }

    /// Issues a DNS query and returns the resulting [`RecordSet`].
    ///
    /// Usually the resulting [`RecordSet`] will belong to the domain name specified in `qname`
    /// parameter. However, if `qname` has a [`CNAME`] record, the RecordSet will belong to
    /// `qname`'s canonical name.
    /// See [`RecordSet::from_msg`] for CNAME chain resolution description.
    ///
    /// This method allows data-type queries only.
    /// For meta-queries (e.g. [`Type::Any`]) use [`query_raw`](Resolver::query_raw).
    ///
    /// This method allocates.
    ///
    /// [`CNAME`]: crate::records::data::Cname
    pub {{ as }} fn query_rrset<D: RData>(&mut self, qname: &str, rclass: RClass) -> Result<RecordSet<D>> {
        self.internal.query_rrset(qname, rclass){{ aw }}
    }
}
