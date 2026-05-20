#![doc = include_str!("../SOCKS4.md")]
use rustls::ClientConfig;
use rustls_pki_types::ServerName;
use std::{marker::PhantomData, net::SocketAddrV4, sync::Arc};
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
pub(crate) struct Reply;

impl TryFrom<u8> for Reply {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            90 => Ok(Reply),
            91 => Err(errors::Error::ProxyResponseNotOk(
                "Request rejected or failed.".into(),
            )),
            92 => Err(errors::Error::ProxyResponseNotOk(
                "Request rejected due to inability to connect to identd on the client.".into(),
            )),
            93 => Err(errors::Error::ProxyResponseNotOk(
                "Request rejected because the client program and identd report different user-IDs."
                    .into(),
            )),
            _ => Err(errors::Error::ProxyResponseNotOk(
                "unknown status reply from proxy server".into(),
            )),
        }
    }
}

/// Guard ensuring that a [`Socks4Builder`] cannot build when option [`Socks4Builder<NoProxy>::proxy`] was absent
pub struct NoProxy;
/// Guard which guarantees that a [`Socks4Builder`] can build because it complied with [`Socks4Builder<NoProxy>::proxy`]
pub struct HasProxy;

/// Builder construct. It guarantees that it's impossible to build into [`Socks4`] when option [`Socks4Builder<NoProxy>::proxy`] was absent
///
/// Be aware that it's always possible to modify underlying proxy and target addresses in [`Socks4`] constructs via [`Socks4::set_proxy`] and [`Socks4::set_target`] respectively
/// Those are also chainable, you can visit their documentation for additional information.
#[derive(Default)]
pub struct Socks4Builder<S = NoProxy> {
    phantom: PhantomData<S>,
    proxy: Option<SocketAddrV4>,
    to: Option<SocketAddrV4>,
}

impl Socks4Builder<NoProxy> {
    /// Construct a [`Socks4Builder`]
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
            proxy: None,
            to: None,
        }
    }

    /// Sets the proxy relay to use
    ///
    /// # Returns
    /// [`Socks4Builder<HasProxy>`], which in turn is given a target destination via [`Socks4Builder<HasProxy>::to`] and eventually calling [`Socks4Builder<HasProxy>::build`] to get a [`Socks4`] instance
    pub fn proxy(self, ipv4: SocketAddrV4) -> Socks4Builder<HasProxy> {
        Socks4Builder {
            phantom: PhantomData,
            proxy: Some(ipv4),
            to: None,
        }
    }
}

impl Socks4Builder<HasProxy> {
    /// Sets the target destination for the proxy relay previously set using  [`Socks4Builder<NoProxy>::to`]
    ///
    /// # Returns
    /// [`Socks4Builder<HasProxy>`], after you should call [`Socks4Builder<HasProxy>::build`] to get a [`Socks4`] instance
    pub fn to(mut self, ipv4: SocketAddrV4) -> Socks4Builder<HasProxy> {
        self.to = Some(ipv4);
        self
    }

    /// Attempts to build into [`Socks4`]
    ///
    /// # Errors
    /// Probable error causes are when called [`Socks4Builder::build`], right away without setting a destination target for the proxy relay using [`Socks4Builder<HasProxy>::to`]
    pub fn build(self) -> crate::Result<Socks4> {
        let proxy = self.proxy.ok_or(errors::Error::SocksProxyAddrNotSet)?;
        let target = self.to.ok_or(errors::Error::SocksTargetAddrNotSet)?;
        Ok(Socks4 { proxy, target })
    }
}

pub struct Socks4 {
    proxy: SocketAddrV4,
    target: SocketAddrV4,
}

impl Socks4 {
    /// Constructs a [`Socks4Builder`] from which you can setup basic configurations and then use [`Socks4Builder<HasProxy>::build`] to construct a [`Socks4`] instance
    pub fn builder() -> Socks4Builder {
        Socks4Builder {
            phantom: PhantomData,
            proxy: None,
            to: None,
        }
    }

    /// Modifies proxy relay server, usually adjacently chained with [`Socks4::set_target`] and [`Socks4::connect`].
    ///
    /// # Caution
    /// Be aware that underlying builder construct already provisioned the proxy server, so use this in cases you need to modify it and not otherwise
    pub fn set_proxy(mut self, ipv4: SocketAddrV4) -> Self {
        self.proxy = ipv4;
        self
    }

    /// Modifies destination target, usually adjacently chained with [`Socks4::set_proxy`] and [`Socks4::connect`]
    ///
    /// # Caution
    /// Be aware that underlying builder construct already provisioned the destination target, so use this in cases you need to modify it and not otherwise
    pub fn set_target(mut self, ipv4: SocketAddrV4) -> Self {
        self.target = ipv4;
        self
    }

    /// Connects to the SOCKS4 proxy server and returns a [TcpStream] which can be used to relay network packets from the proxy server towards destination target
    pub async fn connect(&self) -> crate::Result<TcpStream> {
        let proxy = self.proxy;
        let target = self.target;

        let mut conn = TcpStream::connect(proxy).await?;
        let mut packet = [0u8; 9];

        packet[0] = VERSION_4;
        packet[1] = CONNECT_REQUEST;

        packet[2..4].copy_from_slice(&target.port().to_be_bytes());
        packet[4..8].copy_from_slice(&target.ip().octets());
        packet[8] = NULL;

        conn.write_all(&packet).await?;

        let mut reply = [0u8; 8];
        let n = conn.read(&mut reply[..]).await?;
        if n == 0 {
            return Err(errors::Error::ProxyResponseNotOk(
                "proxy server didn't reply".into(),
            ));
        }

        Reply::try_from(reply[1])?;
        Ok(conn)
    }

    /// Wraps a [`TcpStream`] so that it supports TLS
    ///
    /// - `stream`: [`TcpStream`]
    /// - `config`: [`ClientConfig`]
    /// - `sni`: [`ServerName`]
    #[cfg(feature = "tls")]
    pub async fn tls(
        &self,
        stream: TcpStream,
        config: Arc<ClientConfig>,
        sni: ServerName<'static>,
    ) -> crate::Result<TlsStream<TcpStream>> {
        let connector = TlsConnector::from(config);
        let conn = connector.connect(sni, stream).await?;
        Ok(conn)
    }
}
