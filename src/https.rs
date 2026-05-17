use crate::{errors, is_ok_status};
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

    pub fn build(self) -> Self {
        self
    }

    /// todo
    pub async fn tunnel(
        &self,
        dest: Uri,
        proxy_server: SocketAddrV4,
    ) -> Result<TlsStream<TcpStream>, Box<dyn Error>> {
        let mut conn = TcpStream::connect(proxy_server).await?;
        let connector = TlsConnector::from(self.config.clone().unwrap());
        let dnsname =
            ServerName::try_from(format!("{}", dest.host().ok_or(errors::InvalidHost)?))
                .unwrap();

        let packet = format!(
            "CONNECT {}:{} HTTP/1.1\r\nHost: {}\r\n\r\n",
            dest.host().ok_or(errors::InvalidHost)?,
            dest.port().ok_or(errors::InvalidHost)?,
            dest.host().ok_or(errors::InvalidHost)?
        );

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
            return Err(Box::new(errors::NotOk {
                message: response.into(),
            }));
        }

        let tunnel = connector.connect(dnsname, conn).await?;
        Ok(tunnel)
    }
}