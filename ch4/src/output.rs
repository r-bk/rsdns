use crate::args::Args;
use anyhow::Result;
use chrono::{DateTime, Local};
use rsdns::{
    constants::QType,
    message::{reader::MessageReader, Header},
    resolvers::config::ResolverConfig,
};
use std::{
    fmt::Write,
    time::{Duration, SystemTime},
};

const DOMAIN_NAME_WIDTH: usize = 32;
const QCLASS_WIDTH: usize = 7;
const QTYPE_WIDTH: usize = 7;

#[allow(dead_code)]
pub struct Output<'a, 'b, 'c, 'd> {
    args: &'a Args,
    qname: &'b str,
    qtype: QType,
    msg: &'c [u8],
    ts: SystemTime,
    elapsed: Duration,
    resolver_conf: &'d ResolverConfig,
}

impl<'a, 'b, 'c, 'd> Output<'a, 'b, 'c, 'd> {
    pub fn new(
        args: &'a Args,
        qname: &'b str,
        qtype: QType,
        msg: &'c [u8],
        ts: SystemTime,
        elapsed: Duration,
        resolver_conf: &'d ResolverConfig,
    ) -> Self {
        Self {
            args,
            qname,
            qtype,
            msg,
            ts,
            elapsed,
            resolver_conf,
        }
    }

    pub fn print(&self) -> Result<()> {
        self.print_header();
        self.print_message()?;
        self.print_footer();
        Ok(())
    }

    fn print_message(&self) -> Result<()> {
        let mut mr = MessageReader::new(self.msg)?;
        println!("{}", Self::format_response_header(mr.header())?);
        println!("{}", Self::format_question(&mut mr)?);
        Ok(())
    }

    fn format_response_header(header: &Header) -> Result<String> {
        let mut output = String::new();
        writeln!(
            &mut output,
            ";; ->>HEADER<<- opcode: {}, status: {}, id: {}",
            header.flags.opcode(),
            header.flags.response_code(),
            header.id,
        )?;
        writeln!(
            &mut output,
            ";; flags: {}; QUERY: {}, ANSWER: {}, AUTHORITY: {}, ADDITIONAL: {}",
            Self::format_flags(header),
            header.qd_count,
            header.an_count,
            header.ns_count,
            header.ar_count,
        )?;
        Ok(output)
    }

    fn format_question(mr: &mut MessageReader) -> Result<String> {
        let mut output = String::new();
        writeln!(&mut output, ";; QUESTION SECTION:")?;

        #[allow(clippy::for_loops_over_fallibles)]
        for q in mr.questions() {
            let q = q?;
            let dn_width = DOMAIN_NAME_WIDTH - 2;
            let mut qc_width = QCLASS_WIDTH;
            let mut qt_width = QTYPE_WIDTH;

            if q.qname.len() > dn_width {
                qc_width = 0;
                qt_width = 0;
            }

            writeln!(
                &mut output,
                ";{:dn_width$} {:qc_width$} {:qt_width$}",
                q.qname,
                q.qclass,
                q.qtype,
                dn_width = dn_width,
                qc_width = qc_width,
                qt_width = qt_width,
            )?;
        }
        Ok(output)
    }

    fn format_flags(header: &Header) -> String {
        let mut flags_str = Vec::new();

        if header.flags.message_type().is_response() {
            flags_str.push("qr");
        }
        if header.flags.authoritative_answer() {
            flags_str.push("aa");
        }
        if header.flags.truncated() {
            flags_str.push("tc");
        }
        if header.flags.recursion_desired() {
            flags_str.push("rd");
        }
        if header.flags.recursion_available() {
            flags_str.push("ra");
        }

        flags_str.join(" ")
    }

    fn print_header(&self) {
        println!(
            "; <<>> ch4 {} <<>> {} {}",
            env!("CH4_VERSION"),
            self.qtype,
            self.qname,
        );
    }

    fn print_footer(&self) {
        let datetime: DateTime<Local> = DateTime::from(self.ts);
        println!(";; Query time: {:?}", self.elapsed);
        println!(";; SERVER: {}", self.resolver_conf.nameserver());
        println!(";; WHEN: {}", datetime.to_rfc2822());
        println!(";; MSG SIZE rcvd: {}", self.msg.len());
    }
}
