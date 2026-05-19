use std::net::SocketAddrV4;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tokio_rustls::TlsStream;

static VERSION_4: u8 = 0x04;
static CONNECT_REQUEST: u8 = 0x01;
static NULL: u8 = 0x00;

enum Response {
    Granted = 0x90,
    Failure = 0x91,
    IdentFailure = 0x92,
    IdentInvalid = 0x93,
}

pub async fn tunnel(target: SocketAddrV4, proxy_server: SocketAddrV4) -> crate::Result<TcpStream> {
    let mut conn = TcpStream::connect(proxy_server).await?;
    let mut packet = [0u8; 9];

    packet[0] = VERSION_4;
    packet[1] = CONNECT_REQUEST;

    packet[2..4].copy_from_slice(&target.port().to_be_bytes());
    packet[4..8].copy_from_slice(&target.ip().octets());
    packet[8] = NULL;

    conn.write_all(&packet).await?;
    Ok(conn)
}

pub async fn tunnel_tls(
    target: SocketAddrV4,
    proxy_server: SocketAddrV4,
) -> crate::Result<TlsStream<TcpStream>> {
    let mut conn = TcpStream::connect(proxy_server).await?;
    let mut packet = [0u8; 9];

    packet[0] = VERSION_4;
    packet[1] = CONNECT_REQUEST;

    packet[2..4].copy_from_slice(&target.port().to_be_bytes());
    packet[4..8].copy_from_slice(&target.ip().octets());
    packet[8] = NULL;

    conn.write_all(&packet).await?;
    todo!()
}
