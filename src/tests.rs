use http::Uri;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::test]
async fn test_http_proxy_no_auth() {
    let uri = Uri::from_static("http://httpbin.org:80");
    let mut conn = crate::http::tunnel(&uri, "52.229.30.3:80".parse().unwrap(), None)
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
