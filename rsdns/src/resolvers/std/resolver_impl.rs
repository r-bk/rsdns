use crate::{
    constants::{QClass, QType},
    resolvers::config::ResolverConfig,
    Result,
};

pub(crate) struct ResolverImpl {
    config: ResolverConfig,
}

#[allow(unused_variables)]
impl ResolverImpl {
    pub fn new(config: ResolverConfig) -> Result<Self> {
        Ok(Self { config })
    }

    pub fn config(&self) -> &ResolverConfig {
        &self.config
    }

    pub fn query_raw(
        &self,
        qname: &str,
        qtype: QType,
        qclass: QClass,
        buf: &mut [u8],
    ) -> Result<usize> {
        unimplemented!()
    }
}
