use rustls_pki_types::InvalidDnsNameError;
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid URI")]
    InvalidURI,

    #[error("proxy address was not set")]
    SocksProxyAddrNotSet,

    #[error("target address was not set")]
    SocksTargetAddrNotSet,

    #[error("response from proxy server was not OK: {0}")]
    ProxyResponseNotOk(String),

    #[error("Request rejected or failed.")]
    Socks4ProxyReply0x91,

    #[error("Request rejected due to inability to connect to identd on the client.")]
    Socks4ProxyReply0x92,

    #[error("Request rejected because the client program and identd report different user-IDs.")]
    Socks4ProxyReply0x93,

    #[error(transparent)]
    Other(#[from] std::io::Error),

    #[error(transparent)]
    DNSError(#[from] InvalidDnsNameError),
}
