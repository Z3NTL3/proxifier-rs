//! Auth provides authentication for HTTP/HTTPS and SOCKS4/SOCKS5 proxies.
//!
//! See [`Auth`] for a curated list of available authentication methods.

pub enum Auth {
    /// Use when no authentication is required
    NoAuth,
    /// Header name seperated by semicolon and header value such as `Auth::HTTPAuthorizationHeader("Proxy-Authorization: Basic <encoded>".into())`
    HTTPAuthorizationHeader(String),
    UserPass(String, String),
}
