#### Socks4

This module implements the [SOCKS4](https://www.ietf.org/archive/id/draft-vance-socks-v4-02.html) client-side [CONNECT](https://www.ietf.org/archive/id/draft-vance-socks-v4-02.html#name-connect-operation) operation as defined in the [SOCKS4](https://www.ietf.org/archive/id/draft-vance-socks-v4-02.html) protocol specification.

| OPERATION | DESCRIPTION                                                                                                                  |
| --------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `CONNECT` | The client MUST initiate a CONNECT request when it desires to establish an outbound TCP connection to an application server. |

##### Module-level documentation

This section will make you familiar with the API. Let's start right away!

---

We build into a [`Socks4`] client instance by creating it via a builder construct first. We can do so using [`Socks4Builder`], eventually after building we receive a client in turn:

```rust
 let client = crate::socks4::Socks4::builder()
    .proxy("72.195.34.35:27360".parse().unwrap())
    .to("172.67.74.152:80".parse().unwrap())
    .build()?;
```

Now, we want to connect with the proxy server and make it relay our traffic towards the destination:

```rust
let mut conn = client.connect().await.unwrap();
conn.write(b"GET / HTTP/1.1\r\nHost: api.ipify.org:80\r\nConnection: close\r\n\r\n")
    .await
    .unwrap();

let mut resp = String::new();
conn.read_to_string(&mut resp).await.unwrap();
println!("out: {:?}", resp);
```

That's it. If you ever want to change the proxy relay or destination target you can always do so without having to reconstruct from a builder using:

- [`Socks4::set_proxy`]
- [`Socks4::set_target`]
