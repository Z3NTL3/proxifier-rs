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
            91 => Err(errors::Error::ProxyReply0x91),
            92 => Err(errors::Error::ProxyReply0x92),
            93 => Err(errors::Error::ProxyReply0x93),
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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn proxy(mut self, ipv4: SocketAddrV4) -> Self {
        self.proxy = Some(ipv4);
        self
    }

    pub fn to(mut self, ipv4: SocketAddrV4) -> Self {
        self.to = Some(ipv4);
        self
    }

    pub async fn tunnel(&self) -> crate::Result<TcpStream> {
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

    pub async fn tunnel_tls(
        &self,
        config: Arc<ClientConfig>,
        sni: ServerName<'static>,
    ) -> crate::Result<TlsStream<TcpStream>> {
        let conn = self.tunnel().await?;

        let connector = TlsConnector::from(config);
        let conn = connector.connect(sni, conn).await?;
        Ok(conn)
    }
}
