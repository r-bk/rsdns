use anyhow::Result;
use rsdns::{
    constants::QType,
    resolvers::config::{ProtocolStrategy, Recursion, ResolverConfig},
};
#[cfg(unix)]
use std::io::{BufRead, BufReader};
use std::{
    net::{IpAddr, SocketAddr},
    process::exit,
    str::FromStr,
    time::Duration,
};
use structopt::StructOpt;

#[allow(dead_code)]
pub mod bi {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Debug, StructOpt)]
#[structopt(about = "DNS Stub Resolver", version = env!("CH4_VERSION"))]
pub struct Args {
    #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
    #[structopt(short, long)]
    bind_device: Option<String>,

    #[structopt(short, long, default_value = "53")]
    port: u16,

    #[structopt(
        short = "l",
        long,
        default_value = "10000",
        help = "query lifetime (in msec)."
    )]
    query_lifetime: u64,

    #[structopt(
        short = "t",
        long,
        default_value = "2000",
        help = "query timeout (in msec). Use 0 to disable."
    )]
    query_timeout: u64,

    #[structopt(long, help = "Prints build information")]
    info: bool,

    #[structopt(verbatim_doc_comment)]
    /// Positional arguments ...
    ///
    /// The following arguments may be specified without any particular
    /// order. Arguments specified later take precedence.
    ///
    ///
    /// @<nameserver> - specifies the nameserver IP address. On Unix, if not
    ///                 specified on command line, the first nameserver from
    ///                 /etc/resolv.conf is used.
    ///
    /// <qname>       - domain name to query. Any argument that doesn't match
    ///                 others is considered as such.
    ///
    /// <qtype>       - query type (A, AAAA etc.).
    ///                 Any argument matching any of the supported query types
    ///                 is considered as such.
    ///
    /// +udp          - sets the Udp protocol strategy.
    ///                 UDP is preferred for all queries including ANY.
    ///
    /// +tcp          - sets the Tcp protocol strategy.
    ///                 Only TCP is used for all queries.
    ///
    /// +notcp        - sets NoTcp protocol strategy. When enabled, only UDP
    ///                 is used. Truncated queries are returned as is.
    ///
    /// +[no]rec      - enables (disables) recursive query.
    ///                 Queries are recursive by default.
    pub positional: Vec<String>,
}

impl Args {
    pub fn get() -> Args {
        let args = Args::from_args();

        if args.info {
            Args::show_info();
            exit(0);
        }

        args
    }

    fn show_info() {
        println!("build time:          {}", bi::BUILT_TIME_UTC);
        println!("build semver:        {}", bi::PKG_VERSION);
        println!(
            "git version:         {}",
            bi::GIT_VERSION.or(Some("n/a")).unwrap()
        );
        println!(
            "git commit hash:     {}",
            bi::GIT_COMMIT_HASH.or(Some("n/a")).unwrap()
        );
        println!(
            "git branch:          {}",
            bi::GIT_HEAD_REF.or(Some("n/a")).unwrap()
        );

        println!("compiler:            {}", bi::RUSTC);
        println!("rustc:               {}", bi::RUSTC_VERSION);
        println!(
            "ci platform:         {}",
            bi::CI_PLATFORM.or(Some("n/a")).unwrap()
        );
        println!("cargo features:      {}", bi::FEATURES_STR.to_lowercase());
        println!("cargo profile:       {}", bi::PROFILE);
        println!("cargo target:        {}", bi::TARGET);
        println!("endianness:          {}", bi::CFG_ENDIAN);
        println!("pointer width:       {}", bi::CFG_POINTER_WIDTH);

        println!("build system name:   {}", env!("CH4_SYSINFO_NAME"));
        println!("build os version:    {}", env!("CH4_SYSINFO_OS_VERSION"));
        println!("build cpu vendor:    {}", env!("CH4_SYSINFO_CPU_VENDOR"));
        println!("build cpu brand:     {}", env!("CH4_SYSINFO_CPU_BRAND"));

        cfg_if::cfg_if! {
            if #[cfg(unix)] {
                if let Ok(dns_servers) = Self::load_resolv_conf() {
                    for (index, addr) in dns_servers.iter().enumerate() {
                        println!("dns server #{}:       {}", index, addr);
                    }
                }
            } else if #[cfg(windows)] {
                if let Ok(dns_servers) = crate::win::get_dns_servers() {
                    for (index, addr) in dns_servers.iter().enumerate() {
                        println!("dns server #{}:       {}", index, addr);
                    }
                }
            }
        }
    }

    pub fn parse(&self) -> Result<(ResolverConfig, QType, Vec<String>)> {
        let mut protocol_strategy = ProtocolStrategy::Default;
        let mut nameserver_ip_addr: Option<IpAddr> = None;
        let mut recursion = Recursion::On;
        let mut free_args = Vec::new();
        let mut qtype = QType::A;

        for a in self.positional.iter() {
            match a.as_str() {
                "+udp" => protocol_strategy = ProtocolStrategy::Udp,
                "+tcp" => protocol_strategy = ProtocolStrategy::Tcp,
                "+notcp" => protocol_strategy = ProtocolStrategy::NoTcp,
                "+rec" => recursion = Recursion::On,
                "+norec" => recursion = Recursion::Off,
                s if s.starts_with('@') => match IpAddr::from_str(&s[1..]) {
                    Ok(addr) => nameserver_ip_addr = Some(addr),
                    Err(_) => {
                        eprintln!("failed to parse nameserver ip address");
                        exit(1);
                    }
                },
                s if QType::from_str(&s.to_uppercase()).is_ok() => {
                    qtype = QType::from_str(&s.to_uppercase()).unwrap()
                }
                _ => free_args.push(a.clone()),
            }
        }

        let nameserver = match nameserver_ip_addr {
            Some(addr) => SocketAddr::from((addr, self.port)),
            None => {
                cfg_if::cfg_if! {
                    if #[cfg(unix)] {
                        let nameservers = match Self::load_resolv_conf() {
                            Ok(v) => v,
                            Err(_) => Vec::new(),
                        };

                        if nameservers.is_empty() {
                            eprintln!("no nameservers");
                            exit(1);
                        }

                        SocketAddr::from((nameservers[0], self.port))
                    } else if #[cfg(windows)] {
                        let nameservers = match crate::win::get_dns_servers() {
                            Ok(v) => v,
                            Err(_) => Vec::new(),
                        };

                        if nameservers.is_empty() {
                            eprintln!("no nameservers");
                            exit(1);
                        }

                        nameservers[0]
                    } else {
                        eprintln!("no nameserver");
                        exit(1);
                    }
                }
            }
        };

        #[allow(unused_mut)]
        let mut conf = ResolverConfig::new(nameserver)
            .set_protocol_strategy(protocol_strategy)
            .set_recursion(recursion)
            .set_query_timeout(if self.query_timeout > 0 {
                Some(Duration::from_millis(self.query_timeout))
            } else {
                None
            })
            .set_query_lifetime(Duration::from_millis(self.query_lifetime));

        #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
        if let Some(ref bd) = self.bind_device {
            conf = conf.set_bind_device(Some(bd))?;
        }

        Ok((conf, qtype, free_args))
    }

    #[cfg(unix)]
    fn load_resolv_conf() -> Result<Vec<IpAddr>> {
        let mut addr_list = Vec::new();

        let f = std::fs::File::open("/etc/resolv.conf")?;
        for line in BufReader::new(f).lines() {
            let line = line?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let mut parts = trimmed.split_whitespace();
            if let Some(conf_option) = parts.next() {
                match conf_option {
                    "nameserver" => {
                        if let Some(address) = parts.next() {
                            if let Ok(ip_addr) = IpAddr::from_str(address) {
                                addr_list.push(ip_addr);
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }

        Ok(addr_list)
    }
}
