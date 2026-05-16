### proxifier-rs

A minimal implementation of TCP Forward Proxy

- `http`
- `https`
- `socks4`
- `socks5`

Can do basic password proxy authorization

- Implementation is built around async on top of `tokio`
- `tracing` from Tokio is used, so it's portable to tracing subscribers if you use it

Extensions are partially implemented.
