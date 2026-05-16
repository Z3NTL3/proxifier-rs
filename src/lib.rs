use std::net::SocketAddrV4;

pub enum Protocol {
    SOCKS4(SocketAddrV4),
    SOCKS5(SocketAddrV4),
    HTTP(SocketAddrV4),
    HTTPS(SocketAddrV4),
}

/// todo
pub trait Proxy: Sized {}

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
                "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: Upgrade\r\nUpgrade: connect-ip\r\n\n",
                dest,
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

pub mod https {}
