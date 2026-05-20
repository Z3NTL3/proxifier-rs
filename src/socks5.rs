#![doc = include_str!("../SOCKS4.md")]
use rustls::ClientConfig;
use rustls_pki_types::ServerName;
use std::{marker::PhantomData, net::SocketAddrV4, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::{TlsConnector, client::TlsStream};

use crate::errors;
static VERSION_4: u8 = 0x05;
static CONNECT_REQUEST: u8 = 0x01;
static NULL: u8 = 0x00;

enum ATYP {
    IPV4 = 0x01,
    IPV6 = 0x04,
    DOMAIN = 0x03,
}

enum AuthMethods {
    NoAuth = 0x00,
    UserPass = 0x02,
    NotAcceptable = 0xff,
}

#[derive(Debug)]
pub(crate) struct ReplyOK;

impl TryFrom<u8> for ReplyOK {
    type Error = errors::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ReplyOK),
            1 => Err(Self::Error::Socks5Error(1)),
            2 => Err(Self::Error::Socks5Error(2)),
            3 => Err(Self::Error::Socks5Error(3)),
            4 => Err(Self::Error::Socks5Error(4)),
            5 => Err(Self::Error::Socks5Error(5)),
            6 => Err(Self::Error::Socks5Error(6)),
            7 => Err(Self::Error::Socks5Error(7)),
            8 => Err(Self::Error::Socks5Error(8)),
            9 => Err(Self::Error::Socks5Error(9)),
            10 => Err(Self::Error::Socks5Error(10)),
            _ => Err(errors::Error::ProxyResponseNotOk(
                "unknown status reply from proxy server".into(),
            )),
        }
    }
}
