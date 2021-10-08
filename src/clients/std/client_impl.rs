use crate::{
    clients::config::{ClientConfig, ProtocolStrategy, Recursion},
    constants::{Class, Type},
    errors::{Error, Result},
    message::{reader::MessageIterator, Flags, QueryWriter},
    records::{data::RData, RecordSet},
};
use std::{
    io::{ErrorKind, Read, Write},
    net::{TcpStream, UdpSocket},
    time::{Duration, Instant},
};

const QUERY_BUFFER_SIZE: usize = 288;
type MsgBuf = arrayvec::ArrayVec<u8, QUERY_BUFFER_SIZE>;

struct ClientCtx<'a, 'b, 'c, 'd> {
    qname: &'a str,
    qtype: Type,
    qclass: Class,
    sock: &'b UdpSocket,
    config: &'c ClientConfig,
    msg_id: u16,
    msg: MsgBuf,
    buf: &'d mut [u8],
    start: Instant,
    query_start: Instant,
}

pub(crate) struct ClientImpl {
    config: ClientConfig,
    socket: UdpSocket,
}

impl ClientImpl {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let socket = UdpSocket::bind(config.bind_addr_)?;
        socket.connect(config.nameserver_)?;

        Ok(Self { config, socket })
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub fn query_raw(
        &self,
        qname: &str,
        qtype: Type,
        qclass: Class,
        buf: &mut [u8],
    ) -> Result<usize> {
        let now = Instant::now();
        let mut ctx = ClientCtx {
            qname,
            qtype,
            qclass,
            sock: &self.socket,
            config: &self.config,
            msg_id: 0,
            msg: MsgBuf::default(),
            buf,
            start: now,
            query_start: now,
        };
        ctx.prepare_message()?;
        ctx.query_raw()
    }

    pub fn query_rrset<D: RData>(&self, qname: &str, qclass: Class) -> Result<RecordSet<D>> {
        if !qclass.is_data_class() {
            return Err(Error::UnsupportedClass(qclass));
        }
        let capacity = u16::MAX as usize;
        let mut vec: Vec<u8> = Vec::with_capacity(capacity);
        unsafe { vec.set_len(capacity) };

        let response_len = self.query_raw(qname, D::RTYPE, qclass, &mut vec)?;
        unsafe { vec.set_len(response_len) };

        RecordSet::from_msg(&vec)
    }
}

impl<'a, 'b, 'c, 'd> ClientCtx<'a, 'b, 'c, 'd> {
    #[inline]
    fn query_raw(&mut self) -> Result<usize> {
        match self.query_raw_impl() {
            Err(Error::IoError(v)) if v.kind() == ErrorKind::TimedOut => Err(Error::Timeout),
            Err(Error::IoError(v)) if v.kind() == ErrorKind::WouldBlock => Err(Error::Timeout),
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        }
    }

    fn query_raw_impl(&mut self) -> Result<usize> {
        if self.udp_first() {
            let (size, flags) = self.udp_exchange()?;

            if flags.truncated() && self.tcp_allowed() {
                self.tcp_exchange()
            } else {
                Ok(size)
            }
        } else {
            self.tcp_exchange()
        }
    }

    fn tcp_exchange(&mut self) -> Result<usize> {
        let mut sock = TcpStream::connect_timeout(&self.config.nameserver_, self.lifetime_left()?)?;

        Self::set_timeout_tcp(&sock, self.lifetime_left()?)?;
        sock.write_all(&self.msg)?;

        Self::set_timeout_tcp(&sock, self.lifetime_left()?)?;
        let mut response_size_buf = [0u8; 2];
        sock.read_exact(&mut response_size_buf)?;

        let response_size = u16::from_be_bytes(response_size_buf) as usize;
        if response_size > self.buf.len() {
            return Err(Error::BufferTooShort(response_size));
        }

        Self::set_timeout_tcp(&sock, self.lifetime_left()?)?;
        sock.read_exact(&mut self.buf[..response_size])?;

        Ok(response_size)
    }

    fn udp_exchange(&mut self) -> Result<(usize, Flags)> {
        loop {
            self.query_start = Instant::now();
            Self::set_timeout_udp(self.sock, self.query_left()?)?;

            self.sock.send(&self.msg[2..])?;

            match self.udp_receive_loop() {
                Ok(v) => break Ok(v),
                Err(Error::IoError(v)) if v.kind() == ErrorKind::WouldBlock => continue,
                Err(Error::IoError(v)) if v.kind() == ErrorKind::TimedOut => continue,
                Err(v) => break Err(v),
            }
        }
    }

    fn udp_receive_loop(&mut self) -> Result<(usize, Flags)> {
        loop {
            Self::set_timeout_udp(self.sock, self.query_left()?)?;

            let size = self.sock.recv(&mut self.buf)?;

            let response = &self.buf[..size];
            let mi = match MessageIterator::new(response) {
                Ok(mr) => mr,
                Err(_) => continue,
            };

            if mi.header().id != self.msg_id {
                continue;
            }

            for question in mi.questions().flatten() {
                if question.qtype == self.qtype
                    && question.qclass == self.qclass
                    && question.qname == self.qname
                {
                    return Ok((size, mi.header().flags));
                }
            }
        }
    }

    fn prepare_message(&mut self) -> Result<()> {
        unsafe {
            self.msg.set_len(self.msg.capacity());
        }

        let recursion = self.config.recursion_ == Recursion::On;
        let mut qw = QueryWriter::new(&mut self.msg, recursion);

        self.msg_id = qw.message_id();
        let msg_len = qw.write(self.qname, self.qtype, self.qclass)?;

        unsafe {
            self.msg.set_len(msg_len);
        }

        Ok(())
    }

    #[inline]
    fn udp_first(&self) -> bool {
        match self.config.protocol_strategy_ {
            ProtocolStrategy::Udp | ProtocolStrategy::NoTcp => true,
            ProtocolStrategy::Tcp => false,
        }
    }

    #[inline]
    fn tcp_allowed(&self) -> bool {
        self.config.protocol_strategy_ != ProtocolStrategy::NoTcp
    }

    fn set_timeout_udp(sock: &UdpSocket, timeout: Duration) -> Result<()> {
        let timeout = Some(timeout);
        sock.set_read_timeout(timeout)?;
        sock.set_write_timeout(timeout)?;
        Ok(())
    }

    fn set_timeout_tcp(sock: &TcpStream, timeout: Duration) -> Result<()> {
        let timeout = Some(timeout);
        sock.set_read_timeout(timeout)?;
        sock.set_write_timeout(timeout)?;
        Ok(())
    }

    fn lifetime_left(&self) -> Result<Duration> {
        let elapsed = self.start.elapsed();
        if elapsed >= self.config.query_lifetime_ {
            return Err(Error::Timeout);
        }
        Ok(self.config.query_lifetime_ - elapsed)
    }

    fn query_left(&self) -> Result<Duration> {
        let lifetime_left = self.lifetime_left()?;

        let timeout = match self.config.query_timeout_ {
            Some(duration) => duration,
            _ => self.config.query_lifetime_,
        };

        let elapsed = self.query_start.elapsed();

        let time_left = if elapsed < timeout {
            timeout - elapsed
        } else {
            Duration::from_millis(0)
        };

        Ok(time_left.min(lifetime_left))
    }
}
