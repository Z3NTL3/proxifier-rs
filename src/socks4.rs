use crate::auth;
use byteorder::{BigEndian, ByteOrder};
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
    let mut packet: Vec<u8> = vec![];

    packet.push(VERSION_4); // VN
    packet.push(CONNECT_REQUEST); // CONNECT

    let mut port_bytes = [0u8; 16];
    BigEndian::write_u16(&mut port_bytes, target.port());
    packet.extend_from_slice(&port_bytes);

    let ip = [0u8; 32];
    BigEndian::write_u32(&mut port_bytes, target.port().into());
    packet.extend_from_slice(&ip);

    packet.push(NULL);
    conn.write_all(&packet).await?;

    Ok(conn)
}
