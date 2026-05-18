### proxifier-rs

Proxifier is high-level Proxy Tunnel library. It supports HTTP/HTTPS/SOCKS4/SOCKS5 type proxies.
Built on top of `tokio` and `rustls`.

#### Authentication methods

- `proxifier-rs` can do arbitrary HTTP HEADER authentication such as via `Proxy-Authorization` for `http` and `https` proxy servers
- SOCKS5 supports `NoAuth` and `UserPass` authentication, other methods are not implemented
- SOCKS4 RFC has no builtin authentication support

#### Supports

- `http`
  > Implementation is based on [IP Proxy RFC](#)
- `https`
  > Implementation is based on `CONNECT` in [HTTP RFC](#)
- `socks4`
- `socks5`
