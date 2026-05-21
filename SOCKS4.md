#### Socks4

This module implements the [SOCKS4](https://www.ietf.org/archive/id/draft-vance-socks-v4-02.html) client-side [CONNECT](https://www.ietf.org/archive/id/draft-vance-socks-v4-02.html#name-connect-operation) operation as defined in the [SOCKS4](https://www.ietf.org/archive/id/draft-vance-socks-v4-02.html) protocol specification.

| OPERATION | DESCRIPTION                                                                                                                  |
| --------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `CONNECT` | The client MUST initiate a CONNECT request when it desires to establish an outbound TCP connection to an application server. |

##### Module-level documentation

This section will make you familiar with the API. Let's start right away!

---

A SOCKS4 `CONNECT` stream can be constructed like:

```rust
 let mut conn = crate::socks4::connect(Context {
        proxy: "72.195.34.35:27360".parse().unwrap(),
        destination: "104.26.12.205:80".parse().unwrap(),
    })
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
