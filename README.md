### proxifier-rs

Simple proxy client library to relay network packets towards a destination target using a proxy. With built-in support for TLS.

- Supports SOCKS4/5 and HTTP/HTTPS type proxies

This is a Rust port of [proxifier](https://github.com/z3ntl3/proxifier). Specifically designed for production use with [ProxyBeast](https://github.com/z3ntl3) software.

#### Uses Async

- With `tokio`
- Tracing support todo

#### Supports

- TLS via `rustls`

If you want to use a different TLS library, feel free to disable this crate's default Cargo feature: `tls`

#### Summary of the API

Exports simple `connect` functions from which a [`TcpStream`] is obtained and can in turn be wrapped into a [`TlsStream<TcpStream>`] with support for TLS.

#### Quick glance into the API

```rust
#[tokio::test]
async fn test_socks5_ipv4() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut conn = crate::socks5::connect(
        Context {
            proxy: "194.113.119.68:6742".parse()?,
            destination: NetworkTarget::IP("104.26.12.205:80".parse()?),
        },
        Auth::UserPass("vcilvnba".into(), "vi14viqvvrr7".into()), // or Auth::NoAuth
    )
    .await?;

    conn.write(b"GET / HTTP/1.1\r\nHost: api.ipify.org:80\r\nConnection: close\r\n\r\n")
        .await?;

    let mut resp = String::new();
    conn.read_to_string(&mut resp).await?;
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
