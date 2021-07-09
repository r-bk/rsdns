use crate::{
  constants::{RType, RClass},
  message::Answer,
  resolvers::{
      {{ crate_module_name }}::ResolverImpl,
      config::ResolverConfig,
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
    pub {% if async == "true" %}async {% endif -%} fn new(conf: ResolverConfig) -> Result<Self> {
        Ok(Self {
            internal: ResolverImpl::new(conf){% if async == "true" %}.await{% endif %}?,
        })
    }

    /// Returns the resolver configuration.
    #[inline(always)]
    pub fn config(&self) -> &ResolverConfig {
        self.internal.config()
    }

    /// Issues a DNS query and reads the response into caller-owned buffer `buf`.
    ///
    /// This method gives the control over buffer management to the caller.
    /// The response message is read into `buf` and its length is returned in the result.
    /// This method doesn't allocate.
    #[inline(always)]
    pub {% if async == "true" %}async {% endif -%} fn query_raw(&mut self, qname: &str, qtype: RType, qclass: RClass, buf: &mut [u8]) -> Result<usize> {
        self.internal.query_raw(qname, qtype, qclass, buf){% if async == "true" %}.await{% endif %}
    }

    /// Issues a DNS query and assembles an answer.
    ///
    /// As opposed to [Resolver::query_raw], this method parses the response message and
    /// resolves CNAME chaining if needed.
    ///
    /// Only data record types and classes are allowed.
    /// For meta-queries (e.g. [RType::Any]) use [Resolver::query_raw].
    ///
    /// This method allocates.
    pub {% if async == "true" %}async {% endif -%} fn query(&mut self, qname: &str, rtype: RType, rclass: RClass) -> Result<Answer> {
        self.internal.query(qname, rtype, rclass){% if async == "true" %}.await{% endif %}
    }
}
