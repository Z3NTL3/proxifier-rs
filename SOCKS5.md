#### Socks5

This module implements the [SOCKS5](https://datatracker.ietf.org/doc/html/rfc1928) client-side [CONNECT](https://datatracker.ietf.org/doc/html/rfc1928) operation as defined in the [SOCKS5](https://datatracker.ietf.org/doc/html/rfc1928) protocol specification.

| OPERATION | DESCRIPTION                                                                                                                  |
| --------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `CONNECT` | The client MUST initiate a CONNECT request when it desires to establish an outbound TCP connection to an application server. |

##### Supported auth methods:

- `NoAuth`
- `Username/Password Authentication for SOCKS V5`

#### Supported address types

- for `proxy` only a IPV4
- for `destination` one of: `ipv4`, `ipv6` or `domain name`

##### Module-level documentation

This section will make you familiar with the API. Let's start right away!

---

A SOCKS5 `CONNECT` stream can be constructed like:

```rust
 let mut conn = crate::socks5::connect(
        Context {
            proxy: "194.113.119.68:6742".parse().unwrap(),
            destination: "104.26.12.205:80".parse::<SocketAddrV4>().unwrap().into(),
        },
        Auth::UserPass("vcilvnba".into(), "vi14viqvvrr7".into()),
    )
    .await?;
```

^From this point on, it's upto you what you'll do with the returned [`TcpStream`]:

```rust
conn.write(b"GET / HTTP/1.1\r\nHost: api.ipify.org:80\r\nConnection: close\r\n\r\n")
    .await
    .unwrap();

let mut resp = String::new();
conn.read_to_string(&mut resp).await.unwrap();
println!("out: {:?}", resp);
Ok(())
```

That's it! If you ever want to encapsulate the [`TcpStream`] with `tls` you can use [`crate::socks_with_tls`] or any third party crate.

- Please note that it's possible to disable `tls` artifacts from `rustls` for this package by removing the `tls` feature from crate's default Cargo feature
