#![doc = include_str!("../README.md")]
pub use rustls::{ClientConfig, RootCertStore};
pub use rustls_pki_types::ServerName;
#[cfg(feature = "tls")]
use tokio::net::TcpStream;
#[cfg(feature = "tls")]
use tokio_rustls::{TlsConnector, client::TlsStream};
pub type Result<T> = std::result::Result<T, errors::Error>;

use std::borrow::Cow;
#[cfg(feature = "tls")]
use std::sync::Arc;
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

/// Wraps the [`TcpStream`] so that it supports TLS
///
/// - `stream`: [`TcpStream`]
/// - `config`: [`ClientConfig`]
/// - `sni`: [`ServerName`]
#[cfg(feature = "tls")]
pub async fn socks_tls(
    stream: TcpStream,
    config: Arc<ClientConfig>,
    sni: ServerName<'static>,
) -> crate::Result<TlsStream<TcpStream>> {
    let connector = TlsConnector::from(config);
    let stream = connector.connect(sni, stream).await?;
    Ok(stream)
}
