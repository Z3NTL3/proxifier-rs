pub mod socks4 {}

pub mod socks5 {}

pub mod http {
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
                "CONNECT {}:{} HTTP/1.1\r\nHost: {}\r\n\r\n",
                dest.host().ok_or(InvalidHost)?,
                dest.port().ok_or(InvalidHost)?,
                dest.host().ok_or(InvalidHost)?
            );

            println!("packet {}", packet);

            conn.write_all(packet.as_bytes()).await?;
            conn.flush().await?;

            let mut response = Vec::new();

            loop {
                let mut buf = [0u8; 1024];
                let n = conn.read(&mut buf).await?;

                if n == 0 {
                    println!("proxy closed");
                    break;
                }

                response.extend_from_slice(&buf[..n]);

                // stop after headers
                if response.windows(4).any(|w| w == b"\r\n\r\n") {
                    break;
                }
            }

            println!("resp: {}", String::from_utf8_lossy(&response));

            // tunnel is now ready
            Ok(conn)
        }
    }
}

pub mod https {
    use rustls_pki_types::ServerName;
    use std::{
        error::Error,
        fmt::{self, Display},
        net::SocketAddrV4,
        sync::Arc,
    };
    use tokio_rustls::{TlsConnector, client::TlsStream, rustls::ClientConfig};

    use http::Uri;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpStream,
    };

    pub struct HttpsProxy {
        config: Option<Arc<ClientConfig>>,
    }

    #[derive(Debug)]
    pub struct InvalidHost;

    impl Error for InvalidHost {}
    impl Display for InvalidHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "invalid host")
        }
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
                ServerName::try_from(format!("{}", dest.host().ok_or(InvalidHost)?)).unwrap();

            let packet = format!(
                "CONNECT {}:{} HTTP/1.1\r\nHost: {}\r\n\r\n",
                dest.host().ok_or(InvalidHost)?,
                dest.port().ok_or(InvalidHost)?,
                dest.host().ok_or(InvalidHost)?
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

            let status_ok = String::from_utf8_lossy(&response)
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))
                .and_then(|code| code.parse::<u16>().ok())
                .map(|code| {
                    println!("{}", code);
                    (200..300).contains(&code)
                })
                .unwrap_or(false);

            if !status_ok {
                return Err(Box::new(InvalidHost));
            }

            let tunnel = connector.connect(dnsname, conn).await?;
            Ok(tunnel)
        }
    }
}
