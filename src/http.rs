use crate::{auth, errors, is_ok_status};
use errors::Error;
use http::Uri;
use std::net::SocketAddrV4;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

/// Opens a tunnel via proxy server to target.
///
/// `auth` should be [`auth::Auth::HTTPAuthorizationHeader`], it will be set on Authorization header in initial handshake request to proxy server
///
/// Implementation is based on: [IP Proxy](https://www.rfc-editor.org/rfc/rfc9484.html)
///
/// [crate::https::HttpsProxy] is based on [HTTP CONNECT](https://www.rfc-editor.org/rfc/rfc9110.html#name-connect) and has TLS support via `rustls`
///
///
/// [^note]: You should use [crate::https::HttpsProxy] unless you know what you're doing
pub async fn tunnel(
    dest: Uri,
    proxy_server: SocketAddrV4,
    auth: Option<auth::Auth>,
) -> crate::Result<TcpStream> {
    let mut conn = TcpStream::connect(proxy_server).await?;
    let mut packet = format!(
        "GET {} HTTP/1.1\r\nHost: {}:{}\r\nConnection: Upgrade\r\nUpgrade: connect-ip\r\n",
        dest,
        dest.host().ok_or(Error::InvalidURI)?,
        dest.port().ok_or(Error::InvalidURI)?
    );

    if let Some(auth::Auth::HTTPAuthorizationHeader(header)) = auth {
        packet.push_str(&format!("Authorization: {}\r\n", header));
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
        if response.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }

    let response = String::from_utf8_lossy(&response);
    if !is_ok_status(response.clone()) {
        return Err(Error::ProxyResponseNotOk(response.to_string()));
    }

    Ok(conn)
}
