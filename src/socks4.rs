#![doc = include_str!("../SOCKS4.md")]
use rustls::ClientConfig;
use rustls_pki_types::ServerName;
use std::{net::SocketAddrV4, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::{TlsConnector, client::TlsStream};

use crate::errors;
static VERSION_4: u8 = 0x04;
static CONNECT_REQUEST: u8 = 0x01;
static NULL: u8 = 0x00;

#[derive(Debug)]
pub enum Response {
    Granted = 90,
    Failure = 91,
    IdentFailure = 92,
    IdentInvalid = 93,
}

impl TryFrom<u8> for Response {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            90 => Ok(Response::Granted),
            91 => Err(errors::Error::Socks4ProxyReply0x91),
            92 => Err(errors::Error::Socks4ProxyReply0x92),
            93 => Err(errors::Error::Socks4ProxyReply0x93),
            _ => Err(errors::Error::ProxyResponseNotOk(
                "unknown status reply from proxy server".into(),
            )),
        }
    }
}

#[derive(Default)]
pub struct Socks4 {
    proxy: Option<SocketAddrV4>,
    to: Option<SocketAddrV4>,
}

impl Socks4 {
    /// Constructs a new [`Socks4`] client
    pub fn new() -> Self {
        Default::default()
    }

    /// The proxy server that will relay network packets to outbound destination via [`Socks4::to`]
    pub fn proxy(mut self, ipv4: SocketAddrV4) -> Self {
        self.proxy = Some(ipv4);
        self
    }

    /// Destination address our [`Socks4::proxy`] relays to
    pub fn to(mut self, ipv4: SocketAddrV4) -> Self {
        self.to = Some(ipv4);
        self
    }

    /// Connects to the SOCKS4 proxy server and returns a [TcpStream] which can be used to relay network packets through [`Socks4::proxy`] towards destination target [`Socks4::to`]
    ///
    /// # Composing SOCKS4 Connect request
    /// When you do not set proxy and destination target via [`Socks4::proxy`] and [`Socks4::to`], this function will fail and return the following error: [`errors::Error::SocksProxyAddrNotSet`] or [`errors::Error::SocksTargetAddrNotSet`]
    ///
    /// # Errors:
    /// Have a look at [`errors::Error`] for more information
    ///
    /// # Returns
    /// A ready [`TcpStream`] and already SOCKS4 `CONNECT` established TCP SOCKET
    pub async fn connect(&self) -> crate::Result<TcpStream> {
        let proxy = self.proxy.ok_or(errors::Error::SocksProxyAddrNotSet)?;
        let target = self.to.ok_or(errors::Error::SocksTargetAddrNotSet)?;

        let mut conn = TcpStream::connect(proxy).await?;
        let mut packet = [0u8; 9];

        packet[0] = VERSION_4;
        packet[1] = CONNECT_REQUEST;

        packet[2..4].copy_from_slice(&target.port().to_be_bytes());
        packet[4..8].copy_from_slice(&target.ip().octets());
        packet[8] = NULL;

        conn.write_all(&packet).await?;

        let mut reply = [0u8; 8];
        conn.read(&mut reply[..]).await?;

        Response::try_from(reply[1])?;
        Ok(conn)
    }

    /// Same as [`Socks4::connect`] except for that it wraps the [`TcpStream`] so that it supports TLS
    ///
    /// - `config`: [`ClientConfig`]
    /// - `sni`: [`ServerName`]
    #[cfg(feature = "tls")]
    pub async fn connect_tls(
        &self,
        config: Arc<ClientConfig>,
        sni: ServerName<'static>,
    ) -> crate::Result<TlsStream<TcpStream>> {
        let conn = self.connect().await?;

        let connector = TlsConnector::from(config);
        let conn = connector.connect(sni, conn).await?;
        Ok(conn)
    }
}
