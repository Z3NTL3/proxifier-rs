use std::{
    error::Error,
    fmt::{self, Display},
    net::SocketAddrV4,
};

use http::Uri;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::is_ok_status;

pub struct HttpProxy;

#[derive(Debug)]
pub struct InvalidHost;

impl Error for InvalidHost {}
impl Display for InvalidHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid host")
    }
}

impl HttpProxy {
    /// Will perform HTTP Forward Proxy and return established socket connection
    /// Use [https] if you would like to use HTTP Tunnel with TLS
    pub async fn tunnel(
        dest: Uri,
        proxy_server: SocketAddrV4,
    ) -> Result<TcpStream, Box<dyn Error>> {
        let mut conn = TcpStream::connect(proxy_server).await?;
        let packet = format!(
            "CONNECT {}:80 HTTP/1.1\r\nHost: {}\r\n\r\n",
            dest.host().ok_or(InvalidHost)?,
            dest.host().ok_or(InvalidHost)?
        );

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

        if !is_ok_status(String::from_utf8_lossy(&response)) {}
        Ok(conn)
    }
}
