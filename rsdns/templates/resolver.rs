use crate::{
  constants::{QType, QClass},
  resolvers::{
      {{ crate_module_name }}::ResolverImpl,
      config::ResolverConf,
  },
  Result
};

{% if async == "true" -%}
/// Asynchronous resolver for the [`{{ crate_name }}`](https://crates.io/crates/{{ crate_name }}) executor.
{% else -%}
/// Synchronous (blocking) resolver.
{% endif -%}
pub struct Resolver {
    internal: ResolverImpl,
}

impl Resolver {
    /// Creates a new instance of [Resolver] with provided configuration `conf`.
    #[inline(always)]
    pub {% if async == "true" %}async {% endif -%} fn new(conf: ResolverConf) -> Result<Self> {
        Ok(Self {
            internal: ResolverImpl::new(conf){% if async == "true" %}.await{% endif %}?,
        })
    }

    /// Returns the resolver configuration.
    #[inline(always)]
    pub fn conf(&self) -> &ResolverConf {
        self.internal.conf()
    }

    /// Issues a DNS query and reads the response into caller-owned buffer `buf`.
    ///
    /// This method gives the control over buffer management to the caller.
    /// The response message is read into `buf` and its length is returned in the result.
    /// This method doesn't allocate.
    #[inline(always)]
    pub {% if async == "true" %}async {% endif -%} fn query_raw(&mut self, qname: &str, qtype: QType, qclass: QClass, buf: &mut [u8]) -> Result<usize> {
        self.internal.query_raw(qname, qtype, qclass, buf){% if async == "true" %}.await{% endif %}
    }
}
