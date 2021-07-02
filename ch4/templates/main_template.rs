use crate::{args::Args, output::Output};
use anyhow::Result;
use rsdns::constants::QClass;
use std::time::SystemTime;

{% if async == "true" %}
cfg_if::cfg_if! {
    if #[cfg(feature = "net-tokio")] {
        use rsdns::resolvers::tokio::Resolver;
    } else if #[cfg(feature = "net-async-std")] {
        use rsdns::resolvers::async_std::Resolver;
    } else if #[cfg(feature = "net-smol")] {
        use rsdns::resolvers::smol::Resolver;
    } else {
        compile_error!("One of the async net features must be enabled!!!");
    }
}
{% else %}
use rsdns::resolvers::std::Resolver;
{% endif %}

pub {% if async == "true" %}async{% endif %} fn main() -> Result<()> {
    let mut buf = [0u8; u16::MAX as usize];

    let args = Args::get();
    let (conf, qtype, qnames) = args.parse()?;

    let mut resolver = Resolver::new(conf.clone()){% if async == "true" %}.await{% endif %}?;

    for (index, qname) in qnames.iter().enumerate() {
        let now = SystemTime::now();
        let size = resolver
            .query_raw(qname, qtype, QClass::In, &mut buf){% if async == "true" %}.await{% endif %}?;
        let elapsed = now.elapsed().expect("time failed");


        let output = Output::new(&args, qname, qtype, &buf[..size], now, elapsed, &conf);
        output.print()?;
        if index < qnames.len() - 1 {
            println!();
        }
    }

    Ok(())
}
