use proxifier_rs::{ClientConfig, RootCertStore, socks4::Socks4};
use rustls_pki_types::ServerName;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = Arc::new(
        ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth(),
    );
    let with_sni = ServerName::try_from("api.ipify.org").unwrap();

    let mut conn = Socks4::new()
        .proxy("72.195.34.35:27360".parse().unwrap())
        .to("172.67.74.152:443".parse().unwrap())
        .connect_tls(config.clone(), with_sni)
        .await
        .unwrap();

    conn.write(b"GET / HTTP/1.1\r\nHost: api.ipify.org:443\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut resp = String::new();
    conn.read_to_string(&mut resp).await.unwrap();
    println!("out: {:?}", resp)
}
