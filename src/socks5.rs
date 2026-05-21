#![doc = include_str!("../SOCKS5.md")]
use crate::{
    Context, NetworkTarget,
    auth::{self, Auth},
    errors,
};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
static VERSION_5: u8 = 0x05;
static CONNECT_REQUEST: u8 = 0x01;
static NULL: u8 = 0x00;

static IPV4: u8 = 0x01;
static IPV6: u8 = 0x04;
static DOMAIN: u8 = 0x03;

#[derive(Debug)]
enum AuthMethods {
    NoAuth = 0x00,
    UserPass = 0x02,
}

#[derive(Debug)]
pub(crate) struct AuthReply;

impl TryFrom<u8> for AuthReply {
    type Error = errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 | 2 => Ok(AuthReply),
            255 => Err(errors::Error::ProxyResponseNotOk(
                "preferred auth method rejected".into(),
            )),
            _ => Err(errors::Error::ProxyResponseNotOk(
                "unknown status reply from proxy server".into(),
            )),
        }
    }
}

/// Connects to the SOCKS5 proxy server and returns a [TcpStream] which can be used to relay network packets from the proxy server towards destination target
pub async fn connect(ctx: Context<NetworkTarget>, auth: auth::Auth) -> crate::Result<TcpStream> {
    let mut conn = TcpStream::connect(ctx.proxy).await?;
    let mut packet = [0u8; 3];

    packet[0] = VERSION_5;
    packet[1] = 0x01;
    match auth {
        auth::Auth::UserPass(_, _) => packet[2] = AuthMethods::UserPass as u8,
        _ => packet[2] = AuthMethods::NoAuth as u8,
    }

    conn.write_all(&packet).await?;

    let mut reply = [0u8; 2];
    let n = conn.read(&mut reply[..]).await?;
    if n == 0 {
        return Err(errors::Error::ProxyResponseNotOk(
            "proxy server didn't reply".into(),
        ));
    }

    AuthReply::try_from(reply[1])?;
    let mut auth_req: Vec<u8> = vec![0x01];
    match reply[1] {
        m if m == AuthMethods::NoAuth as u8 => {}
        m if m == AuthMethods::UserPass as u8 => {
            if let Auth::UserPass(user, pass) = auth {
                auth_req.push(user.len() as u8);
                auth_req.extend_from_slice(user.as_bytes());

                auth_req.push(pass.len() as u8);
                auth_req.extend_from_slice(pass.as_bytes());

                conn.write_all(&auth_req).await?;

                let mut reply = [0u8; 2];
                let n = conn.read(&mut reply[..]).await?;
                if n == 0 {
                    return Err(errors::Error::ProxyResponseNotOk(
                        "proxy server didn't reply, it should've verified our user pass request"
                            .into(),
                    ));
                }

                if reply[1] != 0x00 {
                    return Err(errors::Error::ProxyResponseNotOk("auth failure".into()));
                }
            } else {
                // imposible to reach this error as we send request with specific methods, no selection, either NoAuth or UserPass
                // therefore guarantees that proxy server will not randomly choose for us
                return Err(errors::Error::ProxyResponseNotOk(
                    "handshake established for user pass auth but it was not provided by client"
                        .into(),
                ));
            }
        }
        _ => {
            return Err(errors::Error::ProxyResponseNotOk(
                "unknown auth method".into(),
            ));
        }
    }

    let mut packet: Vec<u8> = vec![VERSION_5, CONNECT_REQUEST, NULL];
    let mut remainder_length: usize = 0;

    // ATYP header | dest addr
    match &ctx.destination {
        crate::NetworkTarget::IP(ip) => {
            match ip {
                SocketAddr::V4(addr) => {
                    packet.push(IPV4);
                    packet.extend_from_slice(&addr.ip().octets());
                    remainder_length += 4 * 8; // 32 bits ipv4
                }
                SocketAddr::V6(addr) => {
                    packet.push(IPV6);
                    packet.extend_from_slice(&addr.ip().octets());
                    remainder_length += 16 * 8; // 128 bits ipv6
                }
            }
            packet.extend_from_slice(&ip.port().to_be_bytes());
        }
        crate::NetworkTarget::Domain(domain, port) => {
            packet.push(DOMAIN);
            packet.push(domain.len() as u8);
            packet.extend_from_slice(domain.as_bytes());
            packet.extend_from_slice(&port.0.to_be_bytes());
            remainder_length += domain.len();
        }
    }

    conn.write_all(&mut packet).await?;

    let mut reply = vec![0u8; 6 + remainder_length];
    let n = conn.read(&mut reply).await?;
    if n == 0 {
        return Err(errors::Error::ProxyResponseNotOk(
            "proxy server didn't reply, it should've replied to our CONNECT request".into(),
        ));
    }

    match reply[1] {
        0x00 => Ok(conn),
        0x01 => Err(errors::Error::ProxyResponseNotOk(
            "general SOCKS server failure".into(),
        )),
        0x02 => Err(errors::Error::ProxyResponseNotOk(
            "connection not allowed by ruleset".into(),
        )),
        0x03 => Err(errors::Error::ProxyResponseNotOk(
            "Network unreachable".into(),
        )),
        0x04 => Err(errors::Error::ProxyResponseNotOk("Host unreachable".into())),
        0x05 => Err(errors::Error::ProxyResponseNotOk(
            "Connection refused".into(),
        )),
        0x06 => Err(errors::Error::ProxyResponseNotOk("TTL expired".into())),
        0x07 => Err(errors::Error::ProxyResponseNotOk(
            "Command not supported".into(),
        )),
        0x08 => Err(errors::Error::ProxyResponseNotOk(
            "Address type not supported".into(),
        )),
        0x09 => Err(errors::Error::ProxyResponseNotOk("unassigned x'FF'".into())),
        _ => Err(errors::Error::ProxyResponseNotOk(
            "unknown reply from proxy server on CONNECT request".into(),
        )),
    }
}
