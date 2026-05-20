### proxifier-rs

Proxifier is high-level proxy client library. It supports HTTP/HTTPS/SOCKS4/SOCKS5 type proxies.

- Built on top of `tokio` and `rustls`.
- This is the Rust port for [proxifier](https://proxifier.z3ntl3.com), intended for use with the revamped [ProxyBeast](https://github.com/z3ntl3/ProxyBeast) software

> **Do not use this crate yet, version 0.2.0 will be the final build with complete API, functionality and documentation**

- Final build will have Cargo Features for TLS code artifacts management

##### Features

- `tls`, enabled by default

#### Quick glance into the API

```rust
#[tokio::test]
async fn test_socks4() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut conn = crate::socks4::Socks4::builder()
        .proxy("72.195.34.35:27360".parse().unwrap())
        .to("104.26.12.205:80".parse().unwrap())
        .build()?
        .connect()
        .await?;

    conn.write(b"GET / HTTP/1.1\r\nHost: api.ipify.org:80\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut resp = String::new();
    conn.read_to_string(&mut resp).await.unwrap();
    println!("out: {:?}", resp);
    Ok(())
}

// output:
// "HTTP/1.1 200 OK\r\nDate: Wed, 20 May 2026 11:28:39 GMT\r\nContent-Type: text/plain\r\nContent-Length: 12\r\nConnection: close\r\nServer: cloudflare\r\nVary: Origin\r\ncf-cache-status: DYNAMIC\r\nCF-RAY: 9feaffc71862614d-ATL\r\n\r\n
//
// 72.195.34.35"
```

#### Credits

- [z3ntl3](https://github.com/z3ntl3) (Software Engineer)
- [terzicc](https://terzic.framer.website/) (UI/UX Designer)
  > Contributed to the awesome design of ProxyBeast software and the product landing website for ProxyBeast, which launches soon.
