#![doc = include_str!("../SOCKS5.md")]
use rustls::ClientConfig;
use rustls_pki_types::ServerName;
use std::{
    marker::PhantomData,
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
    sync::Arc,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio_rustls::{TlsConnector, client::TlsStream};

use crate::errors;
static VERSION_5: u8 = 0x05;
static NULL: u8 = 0x00;

/// Represents either IPV4 and IPV6 via [`NetworkAddress::IP`] or domain [`NetworkAddress::Domain`]
pub enum TargetNetwork {
    IP(SocketAddr),
    Domain(String),
}

impl From<SocketAddrV4> for TargetNetwork {
    fn from(value: SocketAddrV4) -> Self {
        Self::IP(SocketAddr::V4(value))
    }
}

impl From<SocketAddrV6> for TargetNetwork {
    fn from(value: SocketAddrV6) -> Self {
        Self::IP(SocketAddr::V6(value))
    }
}

impl From<String> for TargetNetwork {
    fn from(value: String) -> Self {
        Self::Domain(value)
    }
}

impl<'a> From<&'a str> for TargetNetwork {
    fn from(value: &'a str) -> Self {
        Self::Domain(value.into())
    }
}

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

pub struct NoProxy;
pub struct HasProxy;

pub struct Socks5Builder<S = NoProxy> {
    phantom: PhantomData<S>,
}
