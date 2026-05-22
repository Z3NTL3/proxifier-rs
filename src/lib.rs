#![doc = include_str!("../README.md")]
pub use rustls::{ClientConfig, RootCertStore};
pub use rustls_pki_types::ServerName;
#[cfg(feature = "tls")]
use tokio::net::TcpStream;
#[cfg(feature = "tls")]
use tokio_rustls::{TlsConnector, client::TlsStream};
pub type Result<T> = std::result::Result<T, errors::Error>;

#[cfg(feature = "tls")]
use std::sync::Arc;
use std::{
    borrow::Cow,
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
};
fn is_ok_status(utf8_proxy_response: Cow<'_, str>) -> bool {
    utf8_proxy_response
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse::<u16>().ok())
        .map(|code| (200..399).contains(&code))
        .unwrap_or(false)
}

pub mod auth;
pub mod errors;

pub use errors::Error;
pub mod http;
#[cfg(feature = "tls")]
pub mod https;

pub mod socks4;
pub mod socks5;

#[cfg(test)]
mod tests;

/// Represents either IPV4 and IPV6 via [`NetworkTarget::IP`] or domain via [`NetworkTarget::Domain`]
#[derive(Clone)]
pub enum NetworkTarget {
    IP(SocketAddr),
    Domain(String, Port),
}

pub struct Context<T = SocketAddrV4, P = SocketAddrV4> {
    pub destination: T,
    pub proxy: P,
}

/// Used in conjunc tion with [`NetworkTarget::Domain`] and adjacency for SOCKS5 connect operations
///
/// # Example
///
/// ```rust
///  let mut conn = crate::socks5::connect(
///     Context {
///         proxy: "194.113.119.68:6742".parse().unwrap(),
///         destination: NetworkTarget::Domain("api.ipify.org".into(), Port(80)),
///     },
///     Auth::UserPass("vcilvnba".into(), "vi14viqvvrr7".into()),
///  )
///  .await?;
/// ```
#[repr(transparent)]
#[derive(Clone)]
pub struct Port(u16);

impl From<SocketAddrV4> for NetworkTarget {
    fn from(value: SocketAddrV4) -> Self {
        Self::IP(SocketAddr::V4(value))
    }
}

impl From<SocketAddrV6> for NetworkTarget {
    fn from(value: SocketAddrV6) -> Self {
        Self::IP(SocketAddr::V6(value))
    }
}

/// Wraps the [`TcpStream`] so that it supports TLS
///
/// - `stream`: [`TcpStream`]
/// - `config`: [`ClientConfig`]
/// - `sni`: [`ServerName`]
#[cfg(feature = "tls")]
pub async fn socks_with_tls(
    stream: TcpStream,
    config: Arc<ClientConfig>,
    sni: ServerName<'static>,
) -> crate::Result<TlsStream<TcpStream>> {
    let connector = TlsConnector::from(config);
    let stream = connector.connect(sni, stream).await?;
    Ok(stream)
}
