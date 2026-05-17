use std::sync::Arc;

use http::Uri;
use rustls::{ClientConfig, RootCertStore};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    // let mut conn = proxifier_rs::http::HttpProxy::tunnel(
    //     Uri::from_static("https://z3ntl3.com:80"),
    //     "212.58.132.5:8888".parse().unwrap(),
    // )
    // .await
    // .unwrap();

    // conn.write(b"GET / HTTP/1.1\r\nHost: z3ntl3.com:80\r\nConnection: close\r\n\r\n")
    //     .await
    //     .unwrap();

    // let mut out = String::new();
    // conn.read_to_string(&mut out).await.unwrap();
    // println!("out: {}", out)
    //
    //
    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let client = proxifier_rs::https::HttpsProxy::builder()
        .with_client_config(Arc::new(config))
        .build();

    let mut conn = client
        .tunnel(
            Uri::from_static("https://z3ntl3.com:443"),
            "45.92.108.112:80".parse().unwrap(),
        )
        .await
        .unwrap();

    conn.write(b"GET / HTTP/1.1\r\nHost: z3ntl3.com:443\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut out = String::new();
    conn.read_to_string(&mut out).await.unwrap();
    println!("out: {}", out)
}
