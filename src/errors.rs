use rustls_pki_types::InvalidDnsNameError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid URI")]
    InvalidURI,
    #[error("response from proxy server was not OK: {0}")]
    ProxyResponseNotOk(String),

    #[error(transparent)]
    Other(#[from] std::io::Error),

    #[error(transparent)]
    DNSError(#[from] InvalidDnsNameError),
}
