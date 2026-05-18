use base64::prelude::*;
use http::Uri;
use proxifier_rs::{ClientConfig, RootCertStore, auth::Auth};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut conn = proxifier_rs::socks4::tunnel(
        "20.50.2.17:80".parse().unwrap(),
        "36.64.27.123:5678".parse().unwrap(),
    )
    .await
    .unwrap();

    // conn.write(b"GET / HTTP/1.1\r\nHost: z3ntl3.com:80\r\nConnection: close\r\n\r\n")
    //     .await
    //     .unwrap();

    println!("resp");
    let mut out = [0u8; 8];
    conn.read_exact(&mut out).await.unwrap();
    println!("out: {:?}", u8::from(out[1]))
    // let mut root_cert_store = RootCertStore::empty();
    // root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    // let config = ClientConfig::builder()
    //     .with_root_certificates(root_cert_store)
    //     .with_no_client_auth();

    // let client = proxifier_rs::https::HttpsProxy::builder()
    //     .with_client_config(Arc::new(config))
    //     .build()
    //     .unwrap();

    // let mut conn = client
    //     .tunnel(
    //         Uri::from_static("https://z3ntl3.com:443"),
    //         "142.111.48.253:7030".parse().unwrap(),
    //         Some(Auth::HTTPAuthorizationHeader(format!(
    //             "Proxy-Authorization: Basic {}",
    //             BASE64_STANDARD.encode(format!("vcilvnba:vi14viqvvrr7"))
    //         ))),
    //     )
    //     .await
    //     .unwrap();

    // conn.write(b"GET / HTTP/1.1\r\nHost: z3ntl3.com:443\r\nConnection: close\r\n\r\n")
    //     .await
    //     .unwrap();

    // let mut out = String::new();
    // conn.read_to_string(&mut out).await.unwrap();
    // println!("out: {}", out)
}
