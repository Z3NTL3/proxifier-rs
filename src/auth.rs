//! Auth provides authentication for HTTP/HTTPS and SOCKS4/SOCKS5 proxies.
//!
//! See [`Auth`] for a curated list of available authentication methods.

pub enum Auth {
    HTTPAuthorizationHeader(String),
}
