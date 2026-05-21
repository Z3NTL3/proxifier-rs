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
async fn test_socks5_ipv4() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut conn = crate::socks5::connect(
        Context {
            proxy: "194.113.119.68:6742".parse().unwrap(),
            destination: "104.26.12.205:80".parse::<SocketAddrV4>().unwrap().into(),
        },
        Auth::UserPass("vcilvnba".into(), "vi14viqvvrr7".into()),
    )
    .await?;

    conn.write(b"GET / HTTP/1.1\r\nHost: api.ipify.org:80\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut resp = String::new();
    conn.read_to_string(&mut resp).await.unwrap();
    println!("out: {:?}", resp);
    Ok(())
}

// running 1 test
// out: "HTTP/1.1 200 OK\r\nDate: Thu, 21 May 2026 15:15:52 GMT\r\nContent-Type: text/plain\r\nContent-Length: 14\r\nConnection: close\r\nServer: cloudflare\r\nVary: Origin\r\ncf-cache-status: DYNAMIC\r\nCF-RAY: 9ff489fdddc0dc72-FRA\r\n\r\n
//
// 194.113.119.68"
// test tests::test_socks5_ipv4 ... ok
```

#### Credits

- [z3ntl3](https://github.com/z3ntl3) (Software Engineer)
- [terzicc](https://terzic.framer.website/) (UI/UX Designer)
  > Contributed to the awesome design of ProxyBeast software and the product landing website for ProxyBeast, which launches soon.
