pub use crate::auth;
pub use crate::errors;
use crate::is_ok_status;
use http::Uri;
use rustls_pki_types::ServerName;
use std::{error::Error, net::SocketAddrV4, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::{TlsConnector, client::TlsStream, rustls::ClientConfig};

pub struct HttpsProxy {
    config: Arc<ClientConfig>,
}

impl HttpsProxy {
    pub fn with_client_config(config: Arc<ClientConfig>) -> Self {
        Self { config }
    }

    /// Opens a tunnel via proxy server to target.
    ///
    /// `auth` should be [`auth::Auth::HTTPAuthorizationHeader`] or [`None`] when no authentication is required, when given it will be set on Authorization header in initial handshake request to proxy server
    ///
    /// Implementation is based on: [HTTP CONNECT](https://www.rfc-editor.org/rfc/rfc9110.html#name-connect) and has TLS support via `rustls`
    ///
    /// Subtly different, [crate::https::HttpProxy] is an implementation based on [IP Proxy](https://www.rfc-editor.org/rfc/rfc9484.html)
    pub async fn tunnel(
        &self,
        dest: Uri,
        proxy_server: SocketAddrV4,
        auth: Option<auth::Auth>,
    ) -> crate::Result<TlsStream<TcpStream>> {
        let mut conn = TcpStream::connect(proxy_server).await?;
        let connector = TlsConnector::from(self.config.clone());
        let dnsname =
            ServerName::try_from(format!("{}", dest.host().ok_or(errors::Error::InvalidURI)?))?;

        let target = format!(
            "{}:{}",
            dest.host().ok_or(errors::Error::InvalidURI)?,
            dest.port().ok_or(errors::Error::InvalidURI)?
        );
        let mut packet = format!("CONNECT {target} HTTP/1.1\r\nHost: {target}\r\n");
        if let Some(auth::Auth::HTTPAuthorizationHeader(header)) = auth {
            packet.push_str(&format!("{}\r\n", header));
        }

        packet.push_str("\r\n");

        conn.write_all(packet.as_bytes()).await?;
        conn.flush().await?;

        let mut response = Vec::new();
        loop {
            let mut buf = [0u8; 1024];
            let n = conn.read(&mut buf).await?;

            if n == 0 {
                // EOF
                break;
            }

            response.extend_from_slice(&buf[..n]);
            if response.ends_with(b"\r\n\r\n") {
                break;
            }
        }

        let response = String::from_utf8_lossy(&response);
        if !is_ok_status(response.clone()) {
            return Err(errors::Error::ProxyResponseNotOk(response.into()));
        }

        let tunnel = connector.connect(dnsname, conn).await?;
        Ok(tunnel)
    }
}
