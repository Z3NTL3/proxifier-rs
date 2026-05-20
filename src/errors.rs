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

    #[error(transparent)]
    DNSError(#[from] InvalidDnsNameError),

    #[error(transparent)]
    Other(#[from] std::io::Error),
}
