use std::sync::Arc;

use http::Uri;
use rustls::{ClientConfig, RootCertStore};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::auth::Auth;
use base64::prelude::*;

#[tokio::test]
async fn test_http_proxy_auth() {
    let uri = Uri::from_static("http://httpbin.org:80");
    let mut conn = crate::http::tunnel(
        &uri,
        "142.111.48.253:7030".parse().unwrap(),
        Some(Auth::HTTPAuthorizationHeader(format!(
            "Proxy-Authorization: Basic {}",
            BASE64_STANDARD.encode(format!("vcilvnba:vi14viqvvrr7"))
        ))),
    )
    .await
    .unwrap();

    conn.write(
        format!(
            "GET /headers HTTP/1.1\r\nHost: {}:{}\r\nConnection: close\r\n\r\n",
            uri.host().unwrap(),
            uri.port().unwrap()
        )
        .as_bytes(),
    )
    .await
    .unwrap();

    let mut out = String::new();
    conn.read_to_string(&mut out).await.unwrap();
    println!("out: {}", out)
}

#[tokio::test]
async fn test_https_proxy_auth() {
    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let client = crate::https::HttpsProxy::builder()
        .with_client_config(Arc::new(config))
        .build()
        .unwrap();

    let mut conn = client
        .tunnel(
            Uri::from_static("https://z3ntl3.com:443"),
            "142.111.48.253:7030".parse().unwrap(),
            Some(Auth::HTTPAuthorizationHeader(format!(
                "Proxy-Authorization: Basic {}",
                BASE64_STANDARD.encode(format!("vcilvnba:vi14viqvvrr7"))
            ))),
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
