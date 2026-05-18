use http::Uri;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut conn = proxifier_rs::http::tunnel(
        Uri::from_static("http://httpbin.org:80"),
        "52.229.30.3:80".parse().unwrap(),
        None,
    )
    .await
    .unwrap();

    conn.write(b"GET /headers HTTP/1.1\r\nHost: httpbin.org:80\r\nConnection: close\r\n\r\n")
        .await
        .unwrap();

    let mut out = String::new();
    conn.read_to_string(&mut out).await.unwrap();
    println!("out: {}", out)

    //
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
    //         "221.231.13.198:1080".parse().unwrap(),
    //         None,
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
