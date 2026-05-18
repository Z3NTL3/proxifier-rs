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
    config: Option<Arc<ClientConfig>>,
}

impl HttpsProxy {
    pub fn builder() -> Self {
        Self { config: None }
    }

    pub fn with_client_config(mut self, config: Arc<ClientConfig>) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Result<Self, Box<dyn Error>> {
        Ok(self)
    }

    /// todo
    pub async fn tunnel(
        &self,
        dest: Uri,
        proxy_server: SocketAddrV4,
        auth: Option<auth::Auth>,
    ) -> crate::Result<TlsStream<TcpStream>> {
        let mut conn = TcpStream::connect(proxy_server).await?;
        let connector = TlsConnector::from(self.config.clone().unwrap());
        let dnsname =
            ServerName::try_from(format!("{}", dest.host().ok_or(errors::Error::InvalidURI)?))
                .unwrap();

        let target = format!(
            "{}:{}",
            dest.host().ok_or(errors::Error::InvalidURI)?,
            dest.port().ok_or(errors::Error::InvalidURI)?
        );
        let packet = format!("CONNECT {} HTTP/1.1\r\nHost: {}\r\n\r\n", target, target);

        println!("packet {}", packet);

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
            if response.windows(4).any(|w| w == b"\r\n\r\n") {
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
