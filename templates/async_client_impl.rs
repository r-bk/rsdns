use crate::{
    clients::config::{ProtocolStrategy, Recursion, ClientConfig, EDns},
    constants::{Type, Class, DNS_MESSAGE_BUFFER_MIN_LENGTH},
    message::{reader::MessageReader, Flags, QueryWriter},
    records::{data::RData, RecordSet, Opt},
    Error, Result,
};
use std::cell::RefCell;

{% if crate_name == "tokio" %}

    use tokio::{
        net::{TcpStream, UdpSocket},
        io::{AsyncReadExt, AsyncWriteExt},
        time::timeout
    };

    #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
    use std::os::unix::io::{IntoRawFd, FromRawFd};

    #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
    use tokio::net::TcpSocket;

{% elif crate_name == "async-std" %}

    use async_std::{
        future::timeout,
        net::{TcpStream, UdpSocket},
        io::prelude::{ReadExt, WriteExt}
    };

{% elif crate_name == "smol" %}

    use smol::{
        net::{TcpStream, UdpSocket},
        io::{AsyncReadExt, AsyncWriteExt},
    };
    use smol_timeout::TimeoutExt;

{% endif %}

const QUERY_BUFFER_SIZE: usize = 288;
type MsgBuf = cds::arrayvec::ArrayVec<u8, cds::len::U16, cds::mem::Uninitialized, QUERY_BUFFER_SIZE>;

pub struct ClientImpl {
    config: ClientConfig,
    sock: UdpSocket,
    buf: RefCell<Vec<u8>>,
}

impl ClientImpl {
    pub async fn new(config: ClientConfig) -> Result<Self> {
        let sock = udp_socket(&config).await?;
        let buf = match config.buffer_size() {
            0 => Vec::new(),
            bs => Vec::with_capacity(bs),
        };
        Ok(Self { config, sock, buf: RefCell::new(buf) })
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub async fn query_raw(&self, qname: &str, qtype: Type, qclass: Class, buf: &mut [u8]) -> Result<usize> {
        if buf.len() < DNS_MESSAGE_BUFFER_MIN_LENGTH {
            return Err(Error::BufferTooShort(DNS_MESSAGE_BUFFER_MIN_LENGTH));
        }
        let mut ctx = ClientCtx {
            qname,
            qtype,
            qclass,
            sock: &self.sock,
            config: &self.config,
            msg_id: 0,
            msg: MsgBuf::default(),
            buf
        };
        ctx.prepare_message()?;
        ctx.query_raw().await
    }

    pub async fn query_rrset<D: RData>(&mut self, qname: &str, qclass: Class) -> Result<RecordSet<D>> {
        if self.config.buffer_size() == 0 {
            return Err(Error::BadParam("non-zero buffer_size is required"));
        }
        if !qclass.is_data_class() {
            return Err(Error::UnsupportedClass(qclass));
        }

        let mut buf = self.buf.borrow_mut();
        unsafe { buf.set_len(self.config.buffer_size()) };
        let response_len = self.query_raw(qname, D::RTYPE, qclass, &mut buf).await?;
        unsafe { buf.set_len(response_len) };

        RecordSet::from_msg(&buf)
    }
}

struct ClientCtx<'a, 'b, 'c, 'd> {
    qname: &'a str,
    qtype: Type,
    qclass: Class,
    sock: &'b UdpSocket,
    config: &'c ClientConfig,
    msg_id: u16,
    msg: MsgBuf,
    buf: &'d mut [u8],
}

impl<'a, 'b, 'c, 'd> ClientCtx<'a, 'b, 'c, 'd> {
    async fn query_raw(&mut self) -> Result<usize> {
        let query_lifetime = self.config.query_lifetime();

        let future = self.query_raw_impl();

        {% if crate_name == "tokio" or crate_name == "async-std" %}

        match timeout(query_lifetime, future).await {
            Ok(res) => res,
            Err(_) => Err(Error::Timeout),
        }

        {% elif crate_name == "smol" %}

        match future.timeout(query_lifetime).await {
            Some(res) => res,
            None => Err(Error::Timeout),
        }

        {% endif %}
    }

    async fn query_raw_impl(&mut self) -> Result<usize> {
        if self.udp_first() {
            let (size, flags) = self.udp_exchange_loop().await?;

            if flags.truncated() && self.tcp_allowed() {
                self.tcp_exchange().await
            } else {
                Ok(size)
            }
        } else {
            self.tcp_exchange().await
        }
    }

    async fn tcp_exchange(&mut self) -> Result<usize> {
        let mut sock = tcp_socket(self.config).await?;

        sock.write_all(&self.msg).await?;

        let mut response_size_buf = [0u8; 2];
        sock.read_exact(&mut response_size_buf).await?;

        let response_size = u16::from_be_bytes(response_size_buf) as usize;

        if response_size > self.buf.len() {
            return Err(Error::BufferTooShort(response_size));
        }

        sock.read_exact(&mut self.buf[..response_size]).await?;

        Ok(response_size)
    }

    async fn udp_exchange_loop(&mut self) -> Result<(usize, Flags)> {
        loop {
            self.sock.send(&self.msg[2..]).await?;

            let query_timeout = self.config.query_timeout();

            let future = self.udp_receive_loop();

            if let Some(query_timeout) = query_timeout {
                {% if crate_name == "tokio" or crate_name == "async-std" %}

                match timeout(query_timeout, future).await {
                    Ok(res) => return res,
                    Err(_) => continue,
                };

                {% elif crate_name == "smol" %}

                match future.timeout(query_timeout).await {
                    Some(res) => return res,
                    None => continue,
                };

                {% endif %}
            } else {
                return future.await;
            }
        }
    }

    async fn udp_receive_loop(&mut self) -> Result<(usize, Flags)> {
        loop {
            let size = self.sock.recv(self.buf).await?;

            let response = &self.buf[..size];
            let mut mr = match MessageReader::new(response) {
                Ok(mr) => mr,
                Err(_) => continue,
            };
            let header = match mr.header() {
                Ok(h) => h,
                Err(_) => continue,
            };

            if header.id != self.msg_id {
                continue;
            }

            if let Ok(question) = mr.the_question() {
                if question.qtype == self.qtype
                    && question.qclass == self.qclass
                    && question.qname == self.qname {
                    return Ok((size, header.flags));
                }
            }
        }
    }

    fn prepare_message(&mut self) -> Result<()> {
        let opt = match self.config.edns_ {
            EDns::On {
                version,
                udp_payload_size
            } => {
                let ups = (udp_payload_size as usize).min(self.buf.len());
                Some(Opt::new(version, ups as u16))
            },
            EDns::Off => None,
        };
        unsafe { self.msg.set_len(self.msg.capacity()); }
        let mut qw = QueryWriter::new(&mut self.msg);
        self.msg_id = qw.message_id();
        let msg_len = qw.write(self.qname, self.qtype, self.qclass,
                               self.config.recursion_ == Recursion::On, opt)?;
        unsafe { self.msg.set_len(msg_len); }
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
}

{% if crate_name == "tokio" %}

#[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
async fn udp_socket2(config: &ClientConfig) -> Result<UdpSocket> {
    if config.interface_.is_empty() {
        return udp_socket_simple(config).await;
    }

    let mut interface = config.interface_.clone();
    interface.try_push(char::default()).ok(); // add terminating null

    let sock = socket2::Socket::new(
        socket2::Domain::for_address(config.nameserver_),
        socket2::Type::DGRAM.nonblocking().cloexec(),
        Some(socket2::Protocol::UDP)
    )?;

    sock.bind_device(Some(interface.as_bytes()))?;

    let sockaddr = socket2::SockAddr::from(config.bind_addr_);
    sock.bind(&sockaddr)?;

    let sockaddr = socket2::SockAddr::from(config.nameserver_);
    sock.connect(&sockaddr)?;

    let std_sock = unsafe { std::net::UdpSocket::from_raw_fd(sock.into_raw_fd()) };

    Ok(UdpSocket::from_std(std_sock)?)
}

#[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))]
async fn tcp_socket2(config: &ClientConfig) -> Result<TcpStream> {
    if config.interface_.is_empty() {
        return tcp_socket_simple(config).await;
    }

    let mut interface = config.interface_.clone();
    interface.try_push(char::default()).ok(); // add terminating null

    let sock = socket2::Socket::new(
        socket2::Domain::for_address(config.nameserver_),
        socket2::Type::STREAM.nonblocking().cloexec(),
        Some(socket2::Protocol::TCP)
    )?;

    sock.bind_device(Some(interface.as_bytes()))?;
    sock.set_nodelay(true)?;

    let tcp_socket = unsafe { TcpSocket::from_raw_fd(sock.into_raw_fd()) };

    Ok(tcp_socket.connect(config.nameserver_).await?)
}

{% endif %}

#[inline(always)]
async fn udp_socket_simple(config: &ClientConfig) -> Result<UdpSocket> {
    let sock = UdpSocket::bind(config.bind_addr_).await?;
    sock.connect(config.nameserver_).await?;
    Ok(sock)
}

#[inline(always)]
async fn tcp_socket_simple(config: &ClientConfig) -> Result<TcpStream> {
    let sock = TcpStream::connect(config.nameserver_).await?;
    sock.set_nodelay(true)?;
    Ok(sock)
}

#[inline(always)]
async fn udp_socket(config: &ClientConfig) -> Result<UdpSocket> {
    {% if crate_name != "tokio" %}

    udp_socket_simple(config).await

    {% else %}

    cfg_if::cfg_if!{
        if #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))] {
            udp_socket2(config).await
        }
        else {
            udp_socket_simple(config).await
        }
    }

    {% endif %}
}

#[inline(always)]
async fn tcp_socket(config: &ClientConfig) -> Result<TcpStream> {
    {% if crate_name != "tokio" %}

    tcp_socket_simple(config).await

    {% else %}

    cfg_if::cfg_if!{
        if #[cfg(all(target_os = "linux", feature = "net-tokio", feature = "socket2"))] {
            tcp_socket2(config).await
        }
        else {
            tcp_socket_simple(config).await
        }
    }

    {% endif %}
}
