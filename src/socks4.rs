#![doc = include_str!("../SOCKS4.md")]
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{Context, errors};
static VERSION_4: u8 = 0x04;
static CONNECT_REQUEST: u8 = 0x01;
static NULL: u8 = 0x00;

#[derive(Debug)]
pub(crate) struct Reply;

impl TryFrom<u8> for Reply {
    type Error = errors::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            90 => Ok(Reply),
            91 => Err(errors::Error::ProxyResponseNotOk(
                "Request rejected or failed.".into(),
            )),
            92 => Err(errors::Error::ProxyResponseNotOk(
                "Request rejected due to inability to connect to identd on the client.".into(),
            )),
            93 => Err(errors::Error::ProxyResponseNotOk(
                "Request rejected because the client program and identd report different user-IDs."
                    .into(),
            )),
            _ => Err(errors::Error::ProxyResponseNotOk(
                "unknown status reply from proxy server".into(),
            )),
        }
    }
}

/// Connects to the SOCKS4 proxy server and returns a [TcpStream] which can be used to relay network packets from the proxy server towards destination target
pub async fn connect(ctx: Context) -> crate::Result<TcpStream> {
    let proxy = ctx.proxy;
    let target = ctx.destination;

    let mut conn = TcpStream::connect(proxy).await?;
    let mut packet = [0u8; 9];

    packet[0] = VERSION_4;
    packet[1] = CONNECT_REQUEST;

    packet[2..4].copy_from_slice(&target.port().to_be_bytes());
    packet[4..8].copy_from_slice(&target.ip().octets());
    packet[8] = NULL;

    conn.write_all(&packet).await?;

    let mut reply = [0u8; 8];
    let n = conn.read(&mut reply[..]).await?;
    if n == 0 {
        return Err(errors::Error::ProxyResponseNotOk(
            "proxy server didn't reply".into(),
        ));
    }

    Reply::try_from(reply[1])?;
    Ok(conn)
}
