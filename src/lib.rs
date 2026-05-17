use std::borrow::Cow;

pub fn is_ok_status(utf8_proxy_response: Cow<'_, str>) -> bool {
    utf8_proxy_response
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse::<u16>().ok())
        .map(|code| (200..300).contains(&code))
        .unwrap_or(false)
}

pub mod socks4 {}
pub mod socks5 {}

pub mod errors;
pub mod http;
pub mod https;
