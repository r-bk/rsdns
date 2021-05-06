use crate::{
    constants::{QClass, QType},
    resolvers::conf::ResolverConf,
    Result,
};

pub(crate) struct ResolverImpl {
    conf: ResolverConf,
}

#[allow(unused_variables)]
impl ResolverImpl {
    pub fn new(conf: ResolverConf) -> Result<Self> {
        Ok(Self { conf })
    }

    pub fn conf(&self) -> &ResolverConf {
        &self.conf
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
