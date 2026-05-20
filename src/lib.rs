#![doc = include_str!("../README.md")]
pub use rustls::{ClientConfig, RootCertStore};
pub use rustls_pki_types::ServerName;
pub type Result<T> = std::result::Result<T, errors::Error>;

use std::borrow::Cow;
fn is_ok_status(utf8_proxy_response: Cow<'_, str>) -> bool {
    utf8_proxy_response
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse::<u16>().ok())
        .map(|code| (200..399).contains(&code))
        .unwrap_or(false)
}

pub mod auth;
pub mod errors;

pub use errors::Error;
pub mod http;
#[cfg(feature = "tls")]
pub mod https;

pub mod socks4;
pub mod socks5;

#[cfg(test)]
mod tests;
