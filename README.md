### proxifier-rs

Proxifier is high-level Proxy Tunnel library. It supports HTTP/HTTPS/SOCKS4/SOCKS5 type proxies.

- Built on top of `tokio` and `rustls`.
- This is the Rust port for [proxifier](https://proxifier.z3ntl3.com), intended for use with the revamped [ProxyBeast](https://github.com/z3ntl3/ProxyBeast) software

> **Do not use this crate yet, version 0.2.0 will be the final build with complete API, functionality and documentation**

#### Quick glance into the API

```rust
use proxifier_rs::{ClientConfig, RootCertStore, socks4::Socks4};
use rustls_pki_types::ServerName;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = Arc::new(
        ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth(),
    );
    let with_sni = ServerName::try_from("api.ipify.org").unwrap();

    let mut conn = Socks4::new()
        .proxy("72.195.34.35:27360".parse().unwrap())
        .to("172.67.74.152:443".parse().unwrap())
        .tunnel_tls(config.clone(), with_sni)
        .await
        .unwrap();

    conn.write(b"GET / HTTP/1.1\r\nHost: api.ipify.org:443\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut resp = String::new();
    conn.read_to_string(&mut resp).await.unwrap();
    println!("out: {:?}", resp)
}

// output:
// "HTTP/1.1 200 OK\r\nDate: Wed, 20 May 2026 11:28:39 GMT\r\nContent-Type: text/plain\r\nContent-Length: 12\r\nConnection: close\r\nServer: cloudflare\r\nVary: Origin\r\ncf-cache-status: DYNAMIC\r\nCF-RAY: 9feaffc71862614d-ATL\r\n\r\n
//
// 72.195.34.35"
```

#### Credits

- [z3ntl3](https://github.com/z3ntl3) (Software Engineer)
- [terzicc](https://terzic.framer.website/) (UI/UX Designer)
  > Contributed to the awesome design of ProxyBeast software and the documentation site for `proxifier-rs`, which launches very soon
